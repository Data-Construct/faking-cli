use rand::Rng;

use crate::{
    data_faking_bridge::assoc::{get_func_from_string, FNVARI},
    json_reader::{
        obj::{DTObj, FieldsEnum, Obj},
        row::Row,
    },
};

pub fn generate_json_output(grs: &DTObj, n_rows: i64) {
    let mut content = "[\n".to_owned();

    for i in 0..n_rows {
        content.push_str(&loop_through_nested_obj(&grs.schema, 0));

        if i != n_rows - 1 {
            content.push_str(",\n");
        }
    }

    content.push_str("\n]");

    println!("{}", content);

    // content
}

fn loop_through_nested_obj(rows: &Vec<FieldsEnum>, depth: i16) -> String {
    let mut oc = "".to_owned();
    if depth >= 1 {
        oc.push_str("{\n");
    } else {
        oc.push_str("\t{\n");
    }

    for i in 0..rows.len() {
        insert_depth_indentation(&mut oc, depth, "\t\t", "");

        match &rows[i] {
            FieldsEnum::Obj(obj) => {
                oc.push_str("\"");
                oc.push_str(&obj.field_name);
                oc.push_str("\": ");

                if obj.array.active {
                    if obj.null.percentage == 0 {
                        obj_array_generation(&mut oc, &obj, depth);
                    } else {
                        let rand_roll = rand::thread_rng().gen_range(0..100);
                        if obj.null.percentage < rand_roll {
                            obj_array_generation(&mut oc, &obj, depth);
                        } else {
                            insert_null_or_custom_string(&mut oc, &obj.null.str);
                        }
                    }
                } else {
                    if obj.null.percentage == 0 {
                        oc.push_str(loop_through_nested_obj(&obj.fields, depth + 1).as_str());
                    } else {
                        let rand_roll = rand::thread_rng().gen_range(0..100);
                        if obj.null.percentage < rand_roll {
                            oc.push_str(loop_through_nested_obj(&obj.fields, depth + 1).as_str());
                        } else {
                            insert_null_or_custom_string(&mut oc, &obj.null.str);
                        }
                    }
                }
            }

            FieldsEnum::Row(row) => {
                oc.push_str("\"");
                oc.push_str(&row.field_name);
                oc.push_str("\": ");

                if row.array.active {
                    if row.null.percentage == 0 {
                        row_array_generation(&mut oc, row);
                    } else {
                        let rand_roll = rand::thread_rng().gen_range(0..100);
                        if row.null.percentage < rand_roll {
                            row_array_generation(&mut oc, row);
                        } else {
                            insert_null_or_custom_string(&mut oc, &row.null.str);
                        }
                    }
                } else {
                    if row.null.percentage == 0 {
                        create_field_string(&mut oc, &row.generator);
                    } else {
                        let rand_roll = rand::thread_rng().gen_range(0..100);
                        if row.null.percentage < rand_roll {
                            create_field_string(&mut oc, &row.generator);
                        } else {
                            insert_null_or_custom_string(&mut oc, &row.null.str);
                        }
                    }
                }
            }
        }

        if i != rows.len() - 1 {
            oc.push_str(",\n");
        }
    }

    insert_depth_indentation(&mut oc, depth, "\n\t", "}");

    oc
}

#[inline(always)]
fn insert_depth_indentation(oc: &mut String, depth: i16, prefix_str: &str, postfix_str: &str) {
    oc.push_str(prefix_str);
    for _i in 0..depth {
        oc.push_str("\t");
    }
    oc.push_str(postfix_str);
}

#[inline(always)]
fn insert_null_or_custom_string(oc: &mut String, null_string: &Option<String>) {
    match null_string {
        Some(x) => {
            oc.push_str("\"");
            oc.push_str(x.as_str());
            oc.push_str("\"");
        }
        None => oc.push_str("null"),
    }
}

fn obj_array_generation(oc: &mut String, obj: &Obj, depth: i16) {
    let arr_range = rand::thread_rng().gen_range(obj.array.min..obj.array.max);

    insert_depth_indentation(oc, depth, "[\n\t\t\t", "");

    for _a in 0..depth {
        oc.push_str("\t");
    }

    for a in 0..arr_range {
        if obj.array.null.percentage == 0 {
            oc.push_str(loop_through_nested_obj(&obj.fields, depth + 2).as_str());
        } else {
            let rand_roll = rand::thread_rng().gen_range(0..100);
            if obj.array.null.percentage < rand_roll {
                oc.push_str(loop_through_nested_obj(&obj.fields, depth + 2).as_str());
            } else {
                insert_null_or_custom_string(oc, &obj.array.null.str);
            }
        }

        if a != arr_range - 1 {
            insert_depth_indentation(oc, depth, ",\n\t\t\t", "");
        }
    }

    insert_depth_indentation(oc, depth, "\n\t\t", "]");
}

fn row_array_generation(oc: &mut String, row: &Row) {
    let arr_range = rand::thread_rng().gen_range(row.array.min..row.array.max);

    oc.push_str("[");

    for a in 0..arr_range {
        if row.array.null.percentage == 0 {
            create_field_string(oc, &row.generator);
        } else {
            let rand_roll = rand::thread_rng().gen_range(0..100);
            if row.array.null.percentage < rand_roll {
                create_field_string(oc, &row.generator);
            } else {
                insert_null_or_custom_string(oc, &row.array.null.str);
            }
        }

        if a != arr_range - 1 {
            oc.push_str(", ");
        }
    }

    oc.push_str("]");
}

#[inline(always)]
fn create_field_string(oc: &mut String, rs: &String) {
    // TODO(clearfeld): do this once and save the FN_E pointer instead of always doing the lookup
    // this function shouldn't need to take in a &String at all
    let x = get_func_from_string(rs);

    match x {
        FNVARI::I8(f) => {
            oc.push_str(f().to_string().as_str());
        }

        FNVARI::Bool(f) => {
            oc.push_str(f().to_string().as_str());
        }

        FNVARI::String(f) => {
            oc.push_str("\"");
            oc.push_str(f().as_str());
            oc.push_str("\"");
        }
    }
}
