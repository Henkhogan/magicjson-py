import logging
import timeit
import json

#logging.basicConfig(level=logging.DEBUG)
from magicjson import load_file

x = load_file("tests/test1.json")
#logging.getLogger().setLevel(logging.ERROR)

def load_file_rust():
    load_file("tests/test0.json")

def load_file_builtin():
    with open("tests/test0.json", "r") as f:
        json.load(f)

assert(load_file_rust() == load_file_builtin())
"""
y = parse_rust_input(
    input = JsonItem(
        key=None,
        value=None,
        value_custom_type=None,
        value_type=JsonType.Dict,
        items=(
            JsonItem(
                key="key1",
                value="value1",
                items=None,
                value_type=JsonType.String,
                value_custom_type="",
            ),
            JsonItem(
                key="key2",
                value="2",
                items=None,
                value_type=JsonType.Int,
                value_custom_type="",
            ),
            JsonItem(
                key="key3",
                value="3.3",
                items=None,
                value_type=JsonType.Float,
                value_custom_type="",
            ),
            JsonItem(
                key="key4",
                value="true",
                items=None,
                value_type=JsonType.Bool,
                value_custom_type="",
            ),
            JsonItem(
                key="key5",
                value="null",
                items=None,
                value_type=JsonType.Null,
                value_custom_type="",
            ),
            JsonItem(
                key="key6",
                value="",
                items=None,
                value_type=JsonType.CustomType,
                value_custom_type="datetime.datetime",
            ), 
            JsonItem(
                key="key7",
                value=None,
                items=[
                    JsonItem(
                        key=None,
                        value="1",
                        items=None,
                        value_type=JsonType.Int,
                        value_custom_type="",
                    ),
                    JsonItem(
                        key=None,
                        value="2",
                        items=None,
                        value_type=JsonType.Int,
                        value_custom_type="",
                    ),
                    JsonItem(
                        key=None,
                        value="3",
                        items=None,
                        value_type=JsonType.Int,
                        value_custom_type="",
                    ),
                ],
                value_type=JsonType.List,
                value_custom_type="",
            ),
            JsonItem(
                key="key8",
                value=None,
                items=[
                    JsonItem(
                        key="key8.1",
                        value="value1",
                        items=None,
                        value_type=JsonType.String,
                        value_custom_type="",
                    ),
                    JsonItem(
                        key="key8.2",
                        value="5",
                        items=None,
                        value_type=JsonType.Int,
                        value_custom_type="",
                    ),
                    JsonItem(
                        key="key8.3",
                        value=None,
                        items=[
                            JsonItem(
                                key="key8.3.1",
                                value="1",
                                items=None,
                                value_type=JsonType.Int,
                                value_custom_type="",
                            ),
                            JsonItem(
                                key="key8.3.2",
                                value="2",
                                items=None,
                                value_type=JsonType.Int,
                                value_custom_type="",
                            ),
                            JsonItem(
                                key="key8.3.3",
                                value="3",
                                items=None,
                                value_type=JsonType.Int,
                                value_custom_type="",
                            ),
                        ],
                        value_type=JsonType.Dict,
                        value_custom_type="",
                    ),
                ],
                value_type=JsonType.Dict,
                value_custom_type="",
            ),
        ),
    )
)

print(parse_rust_input(x))
"""

print(timeit.timeit(load_file_rust,    number=20000))
print(timeit.timeit(load_file_builtin, number=20000))

