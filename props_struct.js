#!/usr/bin/node

const properties = [
  ['aria-activedescendant', {
    'type': 'id'
  }],
  ['aria-atomic', {
    'type': 'boolean'
  }],
  ['aria-autocomplete', {
    'type': 'token',
    'values': [
      'inline',
      'list',
      'both',
      'none'
    ]
  }],
  ['aria-braillelabel', {
    'type': 'string'
  }],
  ['aria-brailleroledescription', {
    'type': 'string'
  }],
  ['aria-busy', {
    'type': 'boolean'
  }],
  ['aria-checked', {
    'type': 'tristate'
  }],
  ['aria-colcount', {
    type: 'integer',
  }],
  ['aria-colindex', {
    type: 'integer',
  }],
  ['aria-colspan', {
    type: 'integer',
  }],
  ['aria-controls', {
    'type': 'idlist'
  }],
  ['aria-current', {
    type: 'token',
    values: [
      'page',
      'step',
      'location',
      'date',
      'time',
      true,
      false,
    ],
  }],
  ['aria-describedby', {
    'type': 'idlist'
  }],
  ['aria-description', {
    'type': 'string'
  }],
  ['aria-details', {
    'type': 'id'
  }],
  ['aria-disabled', {
    'type': 'boolean'
  }],
  ['aria-dropeffect', {
    'type': 'tokenlist',
    'values': [
      'copy',
      'execute',
      'link',
      'move',
      'none',
      'popup'
    ]
  }],
  ['aria-errormessage', {
    'type': 'id'
  }],
  ['aria-expanded', {
    'type': 'boolean',
    'allowundefined': true
  }],
  ['aria-flowto', {
    'type': 'idlist'
  }],
  ['aria-grabbed', {
    'type': 'boolean',
    'allowundefined': true
  }],
  ['aria-haspopup', {
    'type': 'token',
    'values': [
      false,
      true,
      'menu',
      'listbox',
      'tree',
      'grid',
      'dialog'
    ]
  }],
  ['aria-hidden', {
    'type': 'boolean',
    'allowundefined': true
  }],
  ['aria-invalid', {
    'type': 'token',
    'values': [
      'grammar',
      false,
      'spelling',
      true
    ]
  }],
  ['aria-keyshortcuts', {
    type: 'string',
  }],
  ['aria-label', {
    'type': 'string'
  }],
  ['aria-labelledby', {
    'type': 'idlist'
  }],
  ['aria-level', {
    'type': 'integer'
  }],
  ['aria-live', {
    'type': 'token',
    'values': [
      'assertive',
      'off',
      'polite'
    ]
  }],
  ['aria-modal', {
    type: 'boolean',
  }],
  ['aria-multiline', {
    'type': 'boolean'
  }],
  ['aria-multiselectable', {
    'type': 'boolean'
  }],
  ['aria-orientation', {
    'type': 'token',
    'values': [
      'vertical',
      'undefined',
      'horizontal'
    ]
  }],
  ['aria-owns', {
    'type': 'idlist'
  }],
  ['aria-placeholder', {
    type: 'string',
  }],
  ['aria-posinset', {
    'type': 'integer'
  }],
  ['aria-pressed', {
    'type': 'tristate'
  }],
  ['aria-readonly', {
    'type': 'boolean'
  }],
  ['aria-relevant', {
    'type': 'tokenlist',
    'values': [
      'additions',
      'all',
      'removals',
      'text',
    ]
  }],
  ['aria-required', {
    'type': 'boolean'
  }],
  ['aria-roledescription', {
    type: 'string',
  }],
  ['aria-rowcount', {
    type: 'integer',
  }],
  ['aria-rowindex', {
    type: 'integer',
  }],
  ['aria-rowspan', {
    type: 'integer',
  }],
  ['aria-selected', {
    'type': 'boolean',
    'allowundefined': true
  }],
  ['aria-setsize', {
    'type': 'integer'
  }],
  ['aria-sort', {
    'type': 'token',
    'values': [
      'ascending',
      'descending',
      'none',
      'other'
    ]
  }],
  ['aria-valuemax', {
    'type': 'number'
  }],
  ['aria-valuemin', {
    'type': 'number'
  }],
  ['aria-valuenow', {
    'type': 'number'
  }],
  ['aria-valuetext', {
    'type': 'string'
  }],
];

function map_type(prop_type) {
    switch (prop_type) {
      case "string":
          return "AriaPropType::String";
      case "id":
          return "AriaPropType::Id";
      case "integer":
          return "AriaPropType::Integer";
      case "number":
          return "AriaPropType::Number";
      case "boolean":
          return "AriaPropType::Boolean";
      case "token":
          return "AriaPropType::Token";
      case "tokenlist":
          return "AriaPropType::TokenList";
      case "idlist":
          return "AriaPropType::IdList";
      case "tristate":
          return "AriaPropType::Tristate";
    }
}

console.log("const ARIA_PROP_TYPES: Map<&'static str, AriaPropTypeStruct> = phf_map! {\n");
for (prop of properties) {
    const prop_name = prop[0];
    const conf = prop[1];

    let allow_boolean_values = "false";
    if ("values" in conf) {
        values = "Some(phf_set! { " + conf.values.filter(val => typeof val !== "boolean").map(val => `"${val}"`).join(", ") + "})";
        allow_boolean_values = conf.values.some(val => typeof val === "boolean") ? "true" : "false";
    } else {
        values = "None";
    }

    allow_undefined = conf.allowundefined ? "true" : "false";


    console.log(`"${prop_name}" => AriaPropTypeStruct { prop_type: ${map_type(conf.type)}, allowed_values: ${values}, allow_undefined: ${allow_undefined}, allow_boolean_values: ${allow_boolean_values} },`);
}
console.log("};");
