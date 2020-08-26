use csv::ReaderBuilder;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::iter;

pub struct InputData {
  attribute_names: Vec<String>,
  attribute_map: HashMap<String, Attribute>,
  result_name: String,
  result_variants: Vec<String>,
  rows: Vec<Row>,
}

impl InputData {
  fn new() -> Self {
    InputData {
      attribute_names: Vec::new(),
      attribute_map: HashMap::new(),
      result_name: String::new(),
      result_variants: Vec::new(),
      rows: Vec::new(),
    }
  }

  pub fn from_file(filename: &str, result_position: usize) -> Result<InputData, Box<dyn Error>> {
    let mut data = InputData::new();
    let file = File::open(filename)?;
    let mut csv_input = ReaderBuilder::new().has_headers(false).from_reader(file);
    for (idx, record) in csv_input.records().enumerate() {
      let record = record?;
      let mut row = Row::new();
      if result_position >= record.len() {
        panic!("Result position out of index");
      }
      for (field_idx, field) in record.iter().enumerate() {
        if idx == 0 {
          if field_idx == result_position {
            data.result_name = String::from(field);
          } else {
            data.attribute_names.push(String::from(field));
            let attribute = Attribute::new();
            data.attribute_map.insert(String::from(field), attribute);
          }
        } else {
          if field_idx == result_position {
            row.result = String::from(field);
            if !data.result_variants.contains(&row.result) {
              data.result_variants.push(row.result.clone());
            }
          } else {
            let lookup_idx: usize;
            if field_idx < result_position {
              lookup_idx = field_idx;
            } else {
              lookup_idx = field_idx - 1;
            }
            let attribute_name = data.attribute_names.get(lookup_idx).unwrap();
            let field = String::from(field);
            row.values.insert((*attribute_name).clone(), field.clone());
            let attribute_variants =
              &mut data.attribute_map.get_mut(attribute_name).unwrap().variants;
            if !attribute_variants.contains(&field) {
              attribute_variants.push(field);
            }
          }
        }
      }
      if idx != 0 {
        data.rows.push(row);
      }
    }
    Ok(data)
  }
}

impl Display for InputData {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "Attribute names: {:?}", self.attribute_names)?;
    writeln!(f, "Result name: {}", self.result_name)?;
    writeln!(f, "Result variants: {:?}", self.result_variants)?;
    writeln!(f, "Attributes: {:?}", self.attribute_map)?;
    writeln!(f, "No. of rows: {}", self.rows.len())?;
    Ok(())
  }
}

#[derive(Debug)]
struct Attribute {
  variants: Vec<String>,
}

impl Attribute {
  fn new() -> Self {
    Attribute {
      variants: Vec::new(),
    }
  }
}

struct Row {
  result: String,
  values: HashMap<String, String>,
}

impl Row {
  fn new() -> Self {
    Row {
      result: String::new(),
      values: HashMap::new(),
    }
  }
}

impl Clone for Row {
  fn clone(&self) -> Self {
    let mut values = HashMap::new();
    for (key, value) in self.values.iter() {
      values.insert(key.clone(), value.clone());
    }
    Self {
      result: self.result.clone(),
      values,
    }
  }
}

pub struct DecisionTree {
  attribute: Option<String>,
  leaf_value: Option<String>,
  children: HashMap<String, Box<DecisionTree>>,
  previous_attributes: Vec<String>,
}

impl DecisionTree {
  fn new() -> Self {
    DecisionTree {
      attribute: None,
      leaf_value: None,
      children: HashMap::new(),
      previous_attributes: Vec::new(),
    }
  }
}

impl Display for DecisionTree {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let padding: String = iter::repeat(" ")
      .take(self.previous_attributes.len())
      .collect();
    match &self.attribute {
      None => write!(f, "\n{}{}", padding, self.leaf_value.as_ref().unwrap()),
      Some(attribute) => {
        for (variant, tree) in self.children.iter() {
          write!(f, "\n{}{} -> {} ", padding, attribute, variant)?;
          tree.fmt(f)?;
        }
        Ok(())
      }
    }
  }
}

fn ida3_internal(rows: &[&Row], data: &InputData, tree: &mut DecisionTree) {
  const EPS: f64 = 1e-6;
  let current_entropy = entropy(rows);
  if current_entropy.abs() <= EPS {
    tree.leaf_value = Some(most_common(rows));
    return;
  }
  let attribute = best_attribute(rows, data, &tree.previous_attributes);
  if let Some(attribute) = attribute {
    tree.attribute = Some(attribute.clone());
    let partitioned_rows = partition_by_attribute(rows, data, &attribute);
    for (variant, variant_rows) in partitioned_rows.iter() {
      if variant_rows.len() > 0 {
        let mut sub_tree = DecisionTree::new();
        sub_tree.previous_attributes = tree.previous_attributes.clone();
        sub_tree.previous_attributes.push(attribute.clone());
        ida3_internal(variant_rows, data, &mut sub_tree);
        tree.children.insert((*variant).clone(), Box::new(sub_tree));
      }
    }
  } else {
    tree.leaf_value = Some(most_common(rows));
  }
}

pub fn ida3(data: &InputData) -> DecisionTree {
  let mut tree = DecisionTree::new();
  let mut ref_vec = Vec::new();
  for row in &data.rows {
    ref_vec.push(row);
  }
  ida3_internal(ref_vec.as_slice(), data, &mut tree);
  tree
}

fn most_common(rows: &[&Row]) -> String {
  let counter = count_results(rows);
  let mut max_count = 0;
  let mut max_value = &String::new();
  for (value, count) in counter.iter() {
    if *count >= max_count {
      max_count = *count;
      max_value = *value;
    }
  }
  (*max_value).clone()
}

fn count_results<'a>(rows: &'a [&Row]) -> HashMap<&'a String, i32> {
  let mut counter = HashMap::new();
  for row in rows {
    let before = *counter.get(&row.result).unwrap_or(&0);
    counter.insert(&row.result, before + 1);
  }
  counter
}

fn entropy(rows: &[&Row]) -> f64 {
  let total = rows.len() as f64;
  let mut counter = count_results(rows);
  let mut entropy: f64 = 0.0;
  let most_common_attribute = most_common(rows);
  if counter.contains_key(&String::from("?")) {
    let unknown_count = counter[&String::from("?")];
    counter.remove(&String::from("?"));
    let most_common_count = counter[&most_common_attribute];
    counter.insert(&most_common_attribute, most_common_count + unknown_count);
  }
  for (_, count) in counter.iter() {
    let fraction = *count as f64 / total;
    if *count != 0 {
      entropy += -fraction * fraction.ln();
    }
  }
  entropy
}

fn partition_by_attribute<'a, 'b>(
  rows: &[&'b Row],
  data: &'a InputData,
  attribute: &str,
) -> HashMap<&'a String, Vec<&'b Row>> {
  let mut partitions = HashMap::new();
  for variant in &data.attribute_map.get(attribute).unwrap().variants {
    partitions.insert(variant, Vec::<&Row>::new());
  }
  for row in rows {
    let variant = row.values.get(attribute).unwrap();
    partitions.get_mut(variant).unwrap().push(row);
  }
  return partitions;
}

fn entropy_gain(rows: &[&Row], data: &InputData, attribute: &str) -> f64 {
  let original_entropy = entropy(rows);
  let partitions = partition_by_attribute(rows, data, attribute);
  let total_len = rows.len() as f64;
  let mut total_new_entropy: f64 = 0.0;
  for (_, partitioned_rows) in partitions.iter() {
    let new_entropy = entropy(partitioned_rows.as_slice());
    total_new_entropy += new_entropy * partitioned_rows.len() as f64 / total_len;
  }
  return original_entropy - total_new_entropy;
}

fn best_attribute(
  rows: &[&Row],
  data: &InputData,
  completed_attributes: &[String],
) -> Option<String> {
  let mut max_gain = 0.0;
  let mut best = String::from("");
  for attribute in &data.attribute_names {
    if !completed_attributes.contains(attribute) {
      let gain = entropy_gain(rows, data, attribute);
      if gain >= max_gain {
        max_gain = gain;
        best = attribute.clone();
      }
    }
  }
  if best == String::from("") {
    None
  } else {
    Some(best)
  }
}
