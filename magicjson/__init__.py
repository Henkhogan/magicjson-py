from .magicjson import *

# from typing import Iterable

# def parse_null(input: JsonItem):
#     return None

# def parse_rust_list(input: JsonItem):
#     return tuple([parse_rust_item(item) for item in input.items])
 
# def parse_rust_dict(input: JsonItem):
#     return {item.key: parse_rust_item(item, mandatory_key=True) for item in input.items}

# def parse_rust_string(input: JsonItem):
#     return input.value_str

# def parse_rust_int(input: JsonItem):
#     return input.value_int

# def parse_rust_float(input: JsonItem):
#     return input.value_float

# def parse_rust_bool(input: JsonItem):
#     return input.value_bool

# def parse_rust_custom_type(input: JsonItem):
#     return input.value_custom_type

# def error(input: JsonItem):
#     raise Exception("Invalid value type")

# MAP = {
#     JsonType.Null: parse_null,
#     JsonType.List: parse_rust_list,
#     JsonType.Dict: parse_rust_dict,
#     JsonType.String: parse_rust_string,
#     JsonType.Int: parse_rust_int,
#     JsonType.Float: parse_rust_float,
#     JsonType.Bool: parse_rust_bool,
#     JsonType.CustomType: parse_rust_custom_type,
# }

# def parse_rust_item(item: JsonItem, mandatory_key: bool = False):
#     '''
#         mandatory_key: bool - raises KeyError when item.key is None
#     '''
#     if mandatory_key and item.key is None:
#         raise KeyError(f'key is mandatory for item: {item}')

#     return MAP.get(item.value_type, error)(item)
    

# def parse_rust_input(input: JsonItem):
#     if input.value_type == JsonType.Dict:
#         return parse_rust_dict(input)
#     if input.value_type == JsonType.List:
#         return parse_rust_list(input)
#     raise ValueError("Invalid input type") 
    
