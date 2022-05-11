// Copyright 2018-2022 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::*;
use ink_primitives::StorageKey;

#[test]
fn layout_key_works() {
    let layout_key = LayoutKey::from(&1);
    let json = serde_json::to_string(&layout_key).unwrap();
    assert_eq!(json, "\"0x00000001\"",);
}

fn named_fields_struct_layout(key: &StorageKey) -> Layout {
    StructLayout::new(vec![
        FieldLayout::new("a", CellLayout::new::<i32>(LayoutKey::from(key))),
        FieldLayout::new("b", CellLayout::new::<i64>(LayoutKey::from(key))),
    ])
    .into()
}

#[test]
fn named_fields_work() {
    let layout = named_fields_struct_layout(&345);
    let mut registry = Registry::new();
    let compacted = layout.into_portable(&mut registry);
    let json = serde_json::to_value(&compacted).unwrap();
    let expected = serde_json::json! {
        {
            "struct": {
                "fields": [
                    {
                        "layout": {
                            "cell": {
                                "key": "0x00000159",
                                "ty": 0,
                            }
                        },
                        "name": "a",
                    },
                    {
                        "layout": {
                            "cell": {
                                "key": "0x00000159",
                                "ty": 1,
                            }
                        },
                        "name": "b",
                    }
                ]
            }
        }
    };
    assert_eq!(json, expected);
}

fn tuple_struct_layout(key: &StorageKey) -> Layout {
    StructLayout::new(vec![
        FieldLayout::new(None, CellLayout::new::<i32>(LayoutKey::from(key))),
        FieldLayout::new(None, CellLayout::new::<i64>(LayoutKey::from(key))),
    ])
    .into()
}

#[test]
fn tuple_struct_work() {
    let layout = tuple_struct_layout(&234);
    let mut registry = Registry::new();
    let compacted = layout.into_portable(&mut registry);
    let json = serde_json::to_value(&compacted).unwrap();
    let expected = serde_json::json! {
        {
            "struct": {
                "fields": [
                    {
                        "layout": {
                            "cell": {
                                "key": "0x000000ea",
                                "ty": 0,
                            }
                        },
                        "name": null,
                    },
                    {
                        "layout": {
                            "cell": {
                                "key": "0x000000ea",
                                "ty": 1,
                            }
                        },
                        "name": null,
                    }
                ]
            }
        }
    };
    assert_eq!(json, expected);
}

fn clike_enum_layout(key: &StorageKey) -> Layout {
    EnumLayout::new(
        key,
        vec![
            (Discriminant(0), StructLayout::new(vec![])),
            (Discriminant(1), StructLayout::new(vec![])),
            (Discriminant(2), StructLayout::new(vec![])),
        ],
    )
    .into()
}

#[test]
fn clike_enum_work() {
    let layout = clike_enum_layout(&123);
    let mut registry = Registry::new();
    let compacted = layout.into_portable(&mut registry);
    let json = serde_json::to_value(&compacted).unwrap();
    let expected = serde_json::json! {
        {
            "enum": {
                "dispatchKey": "0x0000007b",
                "variants": {
                    "0": {
                        "fields": [],
                    },
                    "1": {
                        "fields": [],
                    },
                    "2": {
                        "fields": [],
                    },
                }
            }
        }
    };
    assert_eq!(json, expected);
}

fn mixed_enum_layout(key: &StorageKey) -> Layout {
    EnumLayout::new(
        *key,
        vec![
            (Discriminant(0), StructLayout::new(vec![])),
            {
                let variant_key = *key;
                (
                    Discriminant(1),
                    StructLayout::new(vec![
                        FieldLayout::new(
                            None,
                            CellLayout::new::<i32>(LayoutKey::from(variant_key)),
                        ),
                        FieldLayout::new(
                            None,
                            CellLayout::new::<i64>(LayoutKey::from(variant_key)),
                        ),
                    ]),
                )
            },
            {
                let variant_key = *key;
                (
                    Discriminant(2),
                    StructLayout::new(vec![
                        FieldLayout::new(
                            "a",
                            CellLayout::new::<i32>(LayoutKey::from(variant_key)),
                        ),
                        FieldLayout::new(
                            "b",
                            CellLayout::new::<i64>(LayoutKey::from(variant_key)),
                        ),
                    ]),
                )
            },
        ],
    )
    .into()
}

#[test]
fn mixed_enum_work() {
    let layout = mixed_enum_layout(&456);
    let mut registry = Registry::new();
    let compacted = layout.into_portable(&mut registry);
    let json = serde_json::to_value(&compacted).unwrap();
    let expected = serde_json::json! {
        {
            "enum": {
                "dispatchKey": "0x000001c8",
                "variants": {
                    "0": {
                        "fields": [],
                    },
                    "1": {
                        "fields": [
                            {
                                "layout": {
                                    "cell": {
                                        "key": "0x000001c8",
                                        "ty": 0,
                                    }
                                },
                                "name": null,
                            },
                            {
                                "layout": {
                                    "cell": {
                                        "key": "0x000001c8",
                                        "ty": 1,
                                    }
                                },
                                "name": null,
                            }
                        ],
                    },
                    "2": {
                        "fields": [
                            {
                                "layout": {
                                    "cell": {
                                        "key": "0x000001c8",
                                        "ty": 0,
                                    }
                                },
                                "name": "a",
                            },
                            {
                                "layout": {
                                    "cell": {
                                        "key": "0x000001c8",
                                        "ty": 1,
                                    }
                                },
                                "name": "b",
                            }
                        ],
                    },
                }
            }
        }
    };
    assert_eq!(json, expected);
}

fn unbounded_hashing_layout(key: &StorageKey) -> Layout {
    let root_key = key;
    HashLayout::new(
        root_key,
        HashingStrategy::new(
            CryptoHasher::Blake2x256,
            b"ink storage hashmap".to_vec(),
            Vec::new(),
        ),
        CellLayout::new::<(i32, bool)>(LayoutKey::from(root_key)),
    )
    .into()
}

#[test]
fn unbounded_layout_works() {
    let layout = unbounded_hashing_layout(&567);
    let mut registry = Registry::new();
    let compacted = layout.into_portable(&mut registry);
    let json = serde_json::to_value(&compacted).unwrap();
    let expected = serde_json::json! {
        {
            "hash": {
                "layout": {
                    "cell": {
                        "key": "0x00000237",
                        "ty": 0
                    }
                },
                "offset": "0x00000237",
                "strategy": {
                        "hasher": "Blake2x256",
                        "prefix": "0x696e6b2073746f7261676520686173686d6170",
                        "postfix": "",
                }
            }
        }
    };
    assert_eq!(json, expected);
}
