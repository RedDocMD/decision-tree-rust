use std::collections::HashMap;

pub struct InputData {
  attribute_names: Vec<String>,
  attribute_map: HashMap<String, Attribute>,
  rows: Vec<Row>,
}

struct Attribute {
  name: String,
  variants: Vec<String>,
}

struct Row {
  result: bool,
  values: HashMap<String, String>,
}

impl Clone for Row {
  fn clone(&self) -> Self {
    let mut values = HashMap::new();
    for (key, value) in self.values.iter() {
      values.insert(key.clone(), value.clone());
    }
    Self {
      result: self.result,
      values,
    }
  }
}

pub struct DecisionTree {
  attribute: Option<String>,
  leaf_value: Option<String>,
  parent: Option<Box<DecisionTree>>,
  children: HashMap<String, Box<DecisionTree>>,
}

impl DecisionTree {
  pub fn new() -> Self {
    DecisionTree {
      attribute: None,
      leaf_value: None,
      parent: None,
      children: HashMap::new(),
    }
  }
}

fn ida3_internal(data: &InputData, tree: &mut DecisionTree) {}

fn entropy(rows: &[Row]) -> f64 {
  let mut true_count = 0;
  let mut false_count = 0;
  for row in rows {
    if row.result {
      true_count += 1;
    } else {
      false_count += 1;
    }
  }
  let total = rows.len() as f64;
  let true_fraction = true_count as f64 / total;
  let false_fraction = false_count as f64 / total;
  -true_fraction * true_fraction.ln() - false_fraction * false_fraction.ln()
}

fn partition_by_attribute<'a>(
  rows: &[Row],
  data: &'a InputData,
  attribute: &str,
) -> HashMap<&'a String, Vec<Row>> {
  let mut partitions = HashMap::new();
  for variant in &data.attribute_map.get(attribute).unwrap().variants {
    partitions.insert(variant, Vec::<Row>::new());
  }
  for row in rows {
    let variant = row.values.get(attribute).unwrap();
    partitions.get_mut(variant).unwrap().push(row.clone());
  }
  return partitions;
}

fn entropy_gain(rows: &[Row], data: &InputData, attribute: &str) {
  let original_entropy = entropy(rows);
  let partitions = partition_by_attribute(rows, data, attribute);
  let total_len = rows.len() as f64;
  let mut total_new_entropy: f64 = 0.0;
  for (_, partitioned_rows) in partitions.iter() {
    let entropy = entropy(partitioned_rows.as_slice());
  }
}
