#!/bin/env python3

import string


def translate_c_type(type: str) -> str:
    generic_idx = type.find("<")
    generic_type = None
    if generic_idx != -1:
        generic_type = translate_c_type(type[generic_idx + 1 : -1])
        type = type[:generic_idx]

    if type.startswith("const "):
        type = type[6:]

    match type[type.rindex("::") + 2 if "::" in type else 0 :]:
        case "vector" | "deque":
            rust_type = "Vec"
        case "Optional":
            rust_type = "Option"
        case "QString":
            rust_type = "String"
        case "bool":
            rust_type = "bool"
        case "int":
            rust_type = "i32"
        case "unsigned":
            rust_type = "u32"
        case other:
            if other.endswith("_t") and "int" in other[:4]:
                if unsigned := other[0] == "u":
                    other = other[1:]
                rust_type = "iu"[unsigned] + other[3:-2]
            elif other.startswith("q"):
                other = other[1:]
                if unsigned := other[0] == "u":
                    other = other[1:]
                rust_type = "iu"[unsigned] + other[3:]
            elif other[0] in string.ascii_uppercase:
                rust_type = other
            else:
                raise Exception("invalid type " + other)

    if generic_type:
        rust_type += "<" + generic_type + ">"

    return rust_type


def to_snake_case(camel_case: str) -> str:
    return "".join(
        "_" + x.lower() if x in string.ascii_uppercase else x for x in camel_case
    )


for line in open(0).readlines():
    line = line.strip()

    if line.startswith("//"):
        print("/" + line)
        continue
    if line == "":
        print()
        continue

    if line.startswith("Json"):
        type, name, *unused = line[line.index("(") + 1 : line.rindex(")")].split(", ")

        print(to_snake_case(name) + ": " + translate_c_type(type) + ",")
