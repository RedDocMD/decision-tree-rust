use std::collections::HashMap;

pub struct InputData {
  attribute_names: Vec<String>,
  attribute_map: HashMap<String, Attribute>,
  rows: Vec<Row>,
}

struct Attribute {
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
  leaf_value: Option<bool>,
  children: HashMap<String, Box<DecisionTree>>,
  previous_attributes: Vec<String>,
}

impl DecisionTree {
  pub fn new() -> Self {
    DecisionTree {
      attribute: None,
      leaf_value: None,
      children: HashMap::new(),
      previous_attributes: Vec::new(),
    }
  }
}

fn ida3_internal(rows: &[Row], data: &InputData, tree: &mut DecisionTree) {
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
      let mut sub_tree = DecisionTree::new();
      sub_tree.previous_attributes = tree.previous_attributes.clone();
      sub_tree.previous_attributes.push(attribute.clone());
      ida3_internal(variant_rows, data, &mut sub_tree);
      tree.children.insert((*variant).clone(), Box::new(sub_tree));
    }
  } else {
    tree.leaf_value = Some(most_common(rows));
  }
}

pub fn ida3(data: &InputData) -> DecisionTree {
  let mut tree = DecisionTree::new();
  ida3_internal(&data.rows, data, &mut tree);
  tree
}

fn most_common(rows: &[Row]) -> bool {
  let (true_count, false_count) = count_results(rows);
  true_count > false_count
}

fn count_results(rows: &[Row]) -> (i32, i32) {
  let mut true_count = 0;
  let mut false_count = 0;
  for row in rows {
    if row.result {
      true_count += 1;
    } else {
      false_count += 1;
    }
  }
  (true_count, false_count)
}

fn entropy(rows: &[Row]) -> f64 {
  let (true_count, false_count) = count_results(rows);
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

fn entropy_gain(rows: &[Row], data: &InputData, attribute: &str) -> f64 {
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
  rows: &[Row],
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
