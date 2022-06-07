#! /usr/bin/env python3

STRUCT_BASE_NAME = "PlaceholderStructName"
STRUCT_ACTUAL_NAME = "Reg{size}Bits"
FIND_SIZE = 32
DEBUG = 0

REFERENCE_FILE = 'src/reg_reference.rs'
DOC_HIDDEN = '#[doc(hidden)]'

# The comment that gets inserted at the beginning of each automatically
# generated file.
COMMENTS = """
// This file was automatically generated with the `generate_impl_rs.py` script.
// Any bugs in this script should be addressed in the `reg_reference.rs` file.
// 
// The contents of this file is as follows:
// 1. Definition of RegXBits struct
// 2. Implementation of operations
// 3. Implementation of traits
""".strip()

def i_geq_j(_, i, j):
    return i >= j

def sum_of_sizes(size, i, j):
    if i < j:
        return None

    if i + j <= size:
        return i + j

    return None

trait_2arg_impls = {
    "Concat": sum_of_sizes,
}
trait_1arg_impls = {
    "DownCast": i_geq_j,
}

sizes = [ 8, 16, 32, 64, 128 ]

def get_1arg_impl_string(struct, trait, x, y):
    return "impl {struct}{trait}<{y}> for {struct}<{x}> {{}}".format(
        struct=struct,
        trait=trait,
        x=x, y=y,
    )

def get_2arg_impl_string(struct, trait, x, y, outcome):
    return "impl {struct}{trait}<{y}, {outcome}> for {struct}<{x}> {{}}".format(
        struct=struct,
        trait=trait,
        x=x, y=y,
        outcome=outcome
    )

with open(REFERENCE_FILE, 'r') as ref_file:
    for size in sizes:
        REPLACE_INDICATOR = '[REF_REPLACE]'
        STOP_INDICATOR = '[REF_STOP]'

        replace_struct_name = STRUCT_ACTUAL_NAME.format(size = size)

        ref_file.seek(0, 0)
        ref_lines = ref_file.readlines()

        filled_txt = COMMENTS
        filled_txt += "\n"
        print("Generating {} bits struct... ".format(size), end = '')        

        # Fill in the arguments
        for line_num, line in enumerate(ref_lines):
            # Replace the arguments at the beginning of the file
            if REPLACE_INDICATOR in line:
                old_line = line

                line = line.replace(str(FIND_SIZE), str(size))
                if DEBUG == 1:
                    print("Replaced '{FIND_SIZE}' on line {line_num}".format(
                        FIND_SIZE=FIND_SIZE,
                        line_num=line_num+1
                    ))
                    print("From: `{old_line}`".format(old_line=old_line.strip()))
                    print("To: `{line}`".format(line=line.strip()))
                    print("")

            if STRUCT_BASE_NAME in line:
                old_line = line

                num_occurances = 0
                while STRUCT_BASE_NAME in line:
                    line = line.replace(STRUCT_BASE_NAME, replace_struct_name)
                    num_occurances += 1

                if DEBUG == 1:
                    print("Replaced {num_occurances} of '{STRUCT_BASE_NAME}' on line {line_num}".format(
                        num_occurances=num_occurances,
                        STRUCT_BASE_NAME=STRUCT_BASE_NAME,
                        line_num=line_num+1
                    ))
                    print("From: `{old_line}`".format(old_line=old_line.strip()))
                    print("To: `{line}`".format(line=line.strip()))
                    print("")

            if STOP_INDICATOR in line:
                break;

            filled_txt += line

        filled_txt += "\n\n"

        # Loop over all combinations
        for i in range(1, size+1):
            for j in range(1, size+1):
                # 1 argument traits 
                for trait_name, trait_fn in trait_1arg_impls.items():
                    if trait_fn(size, i, j):
                        filled_txt += DOC_HIDDEN
                        filled_txt += "\n"
                        filled_txt += get_1arg_impl_string(replace_struct_name, trait_name, i, j)
                        filled_txt += "\n"

                # 2 argument traits
                for trait_name, trait_fn in trait_2arg_impls.items():
                    outcome = trait_fn(size, i, j)

                    if outcome != None:
                        filled_txt += DOC_HIDDEN
                        filled_txt += "\n"
                        filled_txt += get_2arg_impl_string(replace_struct_name, trait_name, i, j, outcome)
                        filled_txt += "\n"
            
        target_filename = 'src/reg{size}.rs'.format(size=size)
        with open(target_filename, 'w') as target_file:
            target_file.write(filled_txt)

        print("done")