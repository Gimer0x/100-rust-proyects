// Day 95: Project: JSON Merge Tool (Conflict-Aware Merger)
// Build a CLI tool that merges two JSON files, handling nested structures and
// flagging conflicting keys. This is great for configuration managers, data overlays,
// or patching systems.
// Key Concepts:
// + Recursive merging of serde_json::Value
// + Conflict resolution: patch overwrites base
// + Use of Map<String, Value> for JSON objects
// You've created a practical configuration and data merger, extensible to validation,
// diffing, or interactive merges.

// How to merge two JSON files (high level):
// 1. Read base.json and patch.json from disk.
// 2. Parse both as serde_json::Value.
// 3. Call merge_json(&mut base, &patch) so patch is merged into base in-place.
// 4. Write the merged base to output.json.

// Value is the enum that can hold any JSON type (object, array, string, number, bool, null).
use serde_json::Value;
// env gives us command-line arguments (base path, patch path, output path).
use std::env;
// fs is used to read the two input JSON files and write the merged result.
use std::fs;

// Merges patch into base recursively. When both are objects, patch keys are merged in;
// when types differ or are non-objects, base is overwritten by patch.
fn merge_json(base: &mut Value, patch: &Value) {
    // Decide merge strategy by looking at the type of base and patch.
    match (base, patch) {
        // Both are JSON objects: merge patch keys into base (recursive merge).
        (Value::Object(base_map), Value::Object(patch_map)) => {
            // For each key in the patch object, either recurse or insert.
            for (key, patch_value) in patch_map {
                // Debug: print which key we're processing.
                // println!("Key: {} - {}", key, patch_value);
                // Check if this key already exists in base.
                match base_map.get_mut(key) {
                    // Key exists: recurse to merge patch_value into the existing base value.
                    Some(base_value) => merge_json(base_value, patch_value),
                    // Key is new: insert it into base (patch adds this key).
                    None => {
                        // Debug: print when we insert a new key.
                        println!("insert: {} - {}", &key, &patch_value);
                        // Insert the new key and a clone of the patch value into base.
                        base_map.insert(key.clone(), patch_value.clone());
                    }
                }
            }
        }
        // One or both are not objects (e.g. array, string, number): overwrite base with patch.
        (base_value, patch_value) => {
            *base_value = patch_value.clone();
        }
    }
}

fn main() {
    // Collect CLI args so we know which files to read and where to write.
    let args: Vec<String> = env::args().collect();
    // We need exactly 4 args: program name, base.json path, patch.json path, output.json path.
    if args.len() != 4 {
        eprintln!("Usage: {} <base.json> <patch.json> <output.json>", args[0]);
        return;
    }

    // Path to the first JSON file (the base that will be modified).
    let base_path = &args[1];
    // Path to the second JSON file (the patch to merge on top of base).
    let patch_path = &args[2];
    // Path where we will write the merged JSON.
    let output_path = &args[3];

    // Step 1 of merge: read the base file contents as a string.
    let base_data = fs::read_to_string(base_path).expect("Failed to read base.json");
    // Step 2 of merge: read the patch file contents as a string.
    let patch_data = fs::read_to_string(patch_path).expect("Failed to read patch.json");

    // Parse the base string into a JSON Value so we can merge into it.
    let mut base_json: Value = serde_json::from_str(&base_data).expect("Invalid base.json");
    // Parse the patch string into a JSON Value (read-only for the merge).
    let patch_json: Value = serde_json::from_str(&patch_data).expect("Invalid patch.json");

    // Step 3 of merge: merge patch into base; base_json now holds the merged result.
    merge_json(&mut base_json, &patch_json);

    // Step 4 of merge: write the merged JSON to the output file (pretty-printed).
    fs::write(output_path, serde_json::to_string_pretty(&base_json).unwrap())
        .expect("Failed to write merged output");

    // Tell the user where the merged file was saved.
    println!("Merged JSON saved to {}", output_path);
}
