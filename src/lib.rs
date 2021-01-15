mod parser {
    use js_sys::{Function, JsString, Object, Promise};
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "web-tree-sitter")]
    extern {
        #[derive(Clone, Debug)]
        #[wasm_bindgen(extends = Object)]
        pub type Edit;

        // Instance Properties

        #[wasm_bindgen(method, getter, js_name = startIndex)]
        pub fn start_index(this: &Edit) -> u32;

        #[wasm_bindgen(method, getter, js_name = oldEndIndex)]
        pub fn old_end_index(this: &Edit) -> u32;

        #[wasm_bindgen(method, getter, js_name = newEndIndex)]
        pub fn new_end_index(this: &Edit) -> u32;

        #[wasm_bindgen(method, getter, js_name = startPosition)]
        pub fn start_position(this: &Edit) -> Point;

        #[wasm_bindgen(method, getter, js_name = oldEndPosition)]
        pub fn old_end_position(this: &Edit) -> Point;

        #[wasm_bindgen(method, getter, js_name = newEndPosition)]
        pub fn new_end_position(this: &Edit) -> Point;
    }

    pub type Input = Function;

    pub type Logger = Function;

    #[wasm_bindgen(module = "web-tree-sitter")]
    extern {
        #[derive(Clone, Debug)]
        pub type Language;

        // Static Methods

        #[wasm_bindgen(static_method_of = Language)]
        pub fn load(path: &str) -> Promise;

        // Instance Properties

        #[wasm_bindgen(method, getter)]
        pub fn version(this: &Language) -> u32;

        #[wasm_bindgen(method, getter, js_name = fieldCount)]
        pub fn field_count(this: &Language) -> u32;

        // Instance Methods

        #[wasm_bindgen(method, js_name = fieldNameForId)]
        pub fn field_name_for_id(this: &Language, field_id: u32) -> Option<String>;

        #[wasm_bindgen(method, js_name = fieldIdForName)]
        pub fn field_id_for_name(this: &Language, field_name: &str) -> Option<u32>;

        #[wasm_bindgen(method)]
        pub fn query(this: &Language, source: &JsString) -> Query;
    }

    #[wasm_bindgen(module = "web-tree-sitter")]
    extern {
        #[derive(Clone, Debug)]
        #[wasm_bindgen(extends = Object)]
        pub type Options;

        // Instance Properties

        // -> Range[]
        #[wasm_bindgen(method, getter, js_name = includedRanges)]
        pub fn included_ranges(this: &Options) -> Option<Box<[JsValue]>>;
    }

    #[wasm_bindgen(module = "web-tree-sitter")]
    extern {
        #[derive(Clone, Debug)]
        #[wasm_bindgen(extends = Object)]
        pub type Point;

        // Instance Properties

        #[wasm_bindgen(method, getter)]
        pub fn row(this: &Point) -> u32;

        #[wasm_bindgen(method, getter)]
        pub fn column(this: &Point) -> u32;
    }

    #[wasm_bindgen(module = "web-tree-sitter")]
    extern {
        #[derive(Clone, Debug)]
        #[wasm_bindgen(extends = Object)]
        pub type PredicateResult;

        // Instance Properties

        #[wasm_bindgen(method, getter)]
        pub fn operator(this: &PredicateResult) -> String;

        // -> { name: string; type: string }[]
        #[wasm_bindgen(method, getter)]
        pub fn operands(this: &PredicateResult) -> Box<[JsValue]>;
    }

    #[wasm_bindgen(module = "web-tree-sitter")]
    extern {
        #[derive(Clone, Debug)]
        pub type Query;

        // Instance Properties

        #[wasm_bindgen(method, getter, js_name = captureNames)]
        pub fn capture_names(this: &Query) -> Box<[JsValue]>;

        // Instance Methods

        #[wasm_bindgen(method)]
        pub fn delete(this: &Query);

        #[wasm_bindgen(method)]
        pub fn matches(
            this: &Query,
            node: &SyntaxNode,
            start_position: Option<&Point>,
            end_position: Option<&Point>,
        ) -> Box<[JsValue]>;

        #[wasm_bindgen(method)]
        pub fn captures(
            this: &Query,
            node: &SyntaxNode,
            start_position: Option<&Point>,
            end_position: Option<&Point>,
        ) -> Box<[JsValue]>;
    }

    #[wasm_bindgen(module = "web-tree-sitter")]
    extern {
        #[derive(Clone, Debug)]
        #[wasm_bindgen(extends = Object)]
        pub type QueryCapture;

        // Instance Properties

        #[wasm_bindgen(method, getter)]
        pub fn name(this: &QueryCapture) -> String;

        #[wasm_bindgen(method, getter)]
        pub fn node(this: &QueryCapture) -> SyntaxNode;
    }

    #[wasm_bindgen(module = "web-tree-sitter")]
    extern {
        #[derive(Clone, Debug)]
        #[wasm_bindgen(extends = Object)]
        pub type QueryMatch;

        // Instance Properties

        #[wasm_bindgen(method, getter)]
        pub fn pattern(this: &QueryMatch) -> u32;

        // -> QueryCapture[]
        #[wasm_bindgen(method, getter)]
        pub fn captures(this: &QueryMatch) -> Box<[JsValue]>;
    }

    #[wasm_bindgen(module = "web-tree-sitter")]
    extern {
        #[derive(Clone, Debug)]
        #[wasm_bindgen(extends = Object)]
        pub type Range;

        // Instance Properties

        #[wasm_bindgen(method, getter, js_name = startPosition)]
        pub fn start_position(this: &Range) -> Point;

        #[wasm_bindgen(method, getter, js_name = endPosition)]
        pub fn end_position(this: &Range) -> Point;

        #[wasm_bindgen(method, getter, js_name = startIndex)]
        pub fn start_index(this: &Range) -> u32;

        #[wasm_bindgen(method, getter, js_name = endIndex)]
        pub fn end_index(this: &Range) -> u32;
    }

    #[wasm_bindgen(module = "web-tree-sitter")]
    extern {
        #[derive(Clone, Debug)]
        pub type SyntaxNode;

        // Instance Properties

        #[wasm_bindgen(method, getter)]
        pub fn id(this: &SyntaxNode) -> u32;

        #[wasm_bindgen(method, getter)]
        pub fn tree(this: &SyntaxNode) -> Tree;

        #[wasm_bindgen(method, getter, js_name = type)]
        pub fn type_(this: &SyntaxNode) -> JsString;

        #[wasm_bindgen(method, getter)]
        pub fn text(this: &SyntaxNode) -> JsString;

        #[wasm_bindgen(method, getter, js_name = startPosition)]
        pub fn start_position(this: &SyntaxNode) -> Point;

        #[wasm_bindgen(method, getter, js_name = endPosition)]
        pub fn end_position(this: &SyntaxNode) -> Point;

        #[wasm_bindgen(method, getter, js_name = startIndex)]
        pub fn start_index(this: &SyntaxNode) -> u32;

        #[wasm_bindgen(method, getter, js_name = endIndex)]
        pub fn end_index(this: &SyntaxNode) -> u32;

        #[wasm_bindgen(method, getter)]
        pub fn parent(this: &SyntaxNode) -> Option<SyntaxNode>;

        #[wasm_bindgen(method, getter)]
        pub fn children(this: &SyntaxNode) -> Box<[JsValue]>;

        #[wasm_bindgen(method, getter, js_name = namedChildren)]
        pub fn named_children(this: &SyntaxNode) -> Box<[JsValue]>;

        #[wasm_bindgen(method, getter, js_name = childCount)]
        pub fn child_count(this: &SyntaxNode) -> u32;

        #[wasm_bindgen(method, getter, js_name = namedChildCount)]
        pub fn named_child_count(this: &SyntaxNode) -> u32;

        #[wasm_bindgen(method, getter, js_name = firstChild)]
        pub fn first_child(this: &SyntaxNode) -> Option<SyntaxNode>;

        #[wasm_bindgen(method, getter, js_name = firstNamedChild)]
        pub fn first_named_child(this: &SyntaxNode) -> Option<SyntaxNode>;

        #[wasm_bindgen(method, getter, js_name = lastChild)]
        pub fn last_child(this: &SyntaxNode) -> Option<SyntaxNode>;

        #[wasm_bindgen(method, getter, js_name = lastNamedChild)]
        pub fn last_named_child(this: &SyntaxNode) -> Option<SyntaxNode>;

        #[wasm_bindgen(method, getter, js_name = nextSibling)]
        pub fn next_sibling(this: &SyntaxNode) -> Option<SyntaxNode>;

        #[wasm_bindgen(method, getter, js_name = nextNamedSibling)]
        pub fn next_named_sibling(this: &SyntaxNode) -> Option<SyntaxNode>;

        #[wasm_bindgen(method, getter, js_name = previousSibling)]
        pub fn previous_sibling(this: &SyntaxNode) -> Option<SyntaxNode>;

        #[wasm_bindgen(method, getter, js_name = previousNamedSibling)]
        pub fn previous_named_sibling(this: &SyntaxNode) -> Option<SyntaxNode>;

        // Instance Methods

        #[wasm_bindgen(method, js_name = hasChanges)]
        pub fn has_changes(this: &SyntaxNode) -> bool;

        #[wasm_bindgen(method, js_name = hasError)]
        pub fn has_error(this: &SyntaxNode) -> bool;

        #[wasm_bindgen(method)]
        pub fn equals(this: &SyntaxNode, other: &SyntaxNode) -> bool;

        #[wasm_bindgen(method, js_name = isMissing)]
        pub fn is_missing(this: &SyntaxNode) -> bool;

        #[wasm_bindgen(method, js_name = isNamed)]
        pub fn is_named(this: &SyntaxNode) -> bool;

        #[wasm_bindgen(method, js_name = toString)]
        pub fn to_string(this: &SyntaxNode) -> JsString;

        #[wasm_bindgen(method)]
        pub fn child(this: &SyntaxNode, index: u32) -> Option<SyntaxNode>;

        #[wasm_bindgen(method, js_name = namedChild)]
        pub fn named_child(this: &SyntaxNode, index: u32) -> Option<SyntaxNode>;

        #[wasm_bindgen(method, js_name = childForFieldId)]
        pub fn child_for_field_id(this: &SyntaxNode, field_id: u32) -> Option<SyntaxNode>;

        #[wasm_bindgen(method, js_name = childForFieldName)]
        pub fn child_for_field_name(this: &SyntaxNode, field_name: &str) -> Option<SyntaxNode>;

        #[wasm_bindgen(method, js_name = descendantForIndex)]
        pub fn descendant_for_index(this: &SyntaxNode, index: u32) -> SyntaxNode;

        #[wasm_bindgen(method, js_name = descendantForIndex)]
        pub fn descendant_for_index_range(this: &SyntaxNode, start_index: u32, end_index: u32) -> SyntaxNode;

        // -> SyntaxNode[]
        #[wasm_bindgen(method, js_name = descendantOfType)]
        pub fn descendant_of_type_string(
            this: &SyntaxNode,
            type_: &str,
            start_position: Option<&Point>,
            end_position: Option<&Point>,
        ) -> Box<[JsValue]>;

        #[wasm_bindgen(method, js_name = namedDescendantForIndex)]
        pub fn named_descendant_for_index(this: &SyntaxNode, index: u32) -> SyntaxNode;

        #[wasm_bindgen(method, js_name = namedDescendantForIndex)]
        pub fn named_descendant_for_index_range(this: &SyntaxNode, start_index: u32, end_index: u32) -> SyntaxNode;

        #[wasm_bindgen(method, js_name = descendantForPosition)]
        pub fn descendant_for_position(this: &SyntaxNode, position: &Point) -> SyntaxNode;

        #[wasm_bindgen(method, js_name = descendantForPosition)]
        pub fn descendant_for_position_range(
            this: &SyntaxNode,
            start_position: &Point,
            end_position: &Point,
        ) -> SyntaxNode;

        #[wasm_bindgen(method, js_name = namedDescendantForPosition)]
        pub fn named_descendant_for_position(this: &SyntaxNode, position: &Point) -> SyntaxNode;

        #[wasm_bindgen(method, js_name = namedDescendantForPosition)]
        pub fn named_descendant_for_position_range(
            this: &SyntaxNode,
            start_position: &Point,
            end_position: &Point,
        ) -> SyntaxNode;

        #[wasm_bindgen(method)]
        pub fn walk(this: &SyntaxNode) -> TreeCursor;
    }

    #[wasm_bindgen(module = "web-tree-sitter")]
    extern {
        #[derive(Clone, Debug)]
        pub type Tree;

        // Instance Properties

        #[wasm_bindgen(method, getter, js_name = rootNode)]
        pub fn root_node(this: &Tree) -> SyntaxNode;

        // Instance Methods

        #[wasm_bindgen(method)]
        pub fn copy(this: &Tree) -> Tree;

        #[wasm_bindgen(method)]
        pub fn delete(this: &Tree);

        #[wasm_bindgen(method)]
        pub fn edit(this: &Tree, delta: &Edit) -> Tree;

        #[wasm_bindgen(method)]
        pub fn walk(this: &Tree) -> TreeCursor;

        // -> Range[]
        #[wasm_bindgen(method)]
        pub fn get_changed_ranges(this: &Tree, other: &Tree) -> Box<[JsValue]>;

        #[wasm_bindgen(method)]
        pub fn get_edited_ranges(this: &Tree, other: &Tree) -> Range;

        #[wasm_bindgen(method)]
        pub fn get_language(this: &Tree) -> Language;
    }

    #[wasm_bindgen(module = "web-tree-sitter")]
    extern {
        #[derive(Clone, Debug)]
        pub type TreeCursor;

        // Instance Properties

        #[wasm_bindgen(method, getter, js_name = nodeType)]
        pub fn node_type(this: &TreeCursor) -> JsString;

        #[wasm_bindgen(method, getter, js_name = nodeText)]
        pub fn node_text(this: &TreeCursor) -> JsString;

        #[wasm_bindgen(method, getter, js_name = nodeIsNamed)]
        pub fn node_is_named(this: &TreeCursor) -> bool;

        #[wasm_bindgen(method, getter, js_name = startPosition)]
        pub fn start_position(this: &TreeCursor) -> Point;

        #[wasm_bindgen(method, getter, js_name = endPosition)]
        pub fn end_position(this: &TreeCursor) -> Point;

        #[wasm_bindgen(method, getter, js_name = startPosition)]
        pub fn start_index(this: &TreeCursor) -> u32;

        #[wasm_bindgen(method, getter, js_name = endPosition)]
        pub fn end_index(this: &TreeCursor) -> u32;

        // Instance Methods

        #[wasm_bindgen(method)]
        pub fn reset(this: &TreeCursor, node: &SyntaxNode);

        #[wasm_bindgen(method)]
        pub fn delete(this: &TreeCursor);

        #[wasm_bindgen(method, js_name = currentNode)]
        pub fn current_node(this: &TreeCursor) -> SyntaxNode;

        #[wasm_bindgen(method, js_name = currentFieldId)]
        pub fn current_field_id(this: &TreeCursor) -> u32;

        #[wasm_bindgen(method, js_name = currentFieldName)]
        pub fn current_field_name(this: &TreeCursor) -> JsString;

        #[wasm_bindgen(method, js_name = gotoParent)]
        pub fn goto_parent(this: &TreeCursor) -> bool;

        #[wasm_bindgen(method, js_name = gotoFirstChild)]
        pub fn goto_first_child(this: &TreeCursor) -> bool;

        #[wasm_bindgen(method, js_name = gotoFirstChildForIndex)]
        pub fn goto_first_child_for_index(this: &TreeCursor, index: u32) -> bool;

        #[wasm_bindgen(method, js_name = gotoNextSibling)]
        pub fn goto_next_sibling(this: &TreeCursor) -> bool;
    }
}

use js_sys::{JsString, Promise};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "web-tree-sitter")]
extern {
    #[derive(Clone, Debug)]
    pub type Parser;

    // Static Methods

    // -> Promise<void>
    #[wasm_bindgen(static_method_of = Parser)]
    pub fn init() -> Promise;

    // Instance Methods

    #[wasm_bindgen(method)]
    pub fn delete(this: &Parser);

    #[wasm_bindgen(method, js_name = parse)]
    pub fn parse_with_function(
        this: &Parser,
        input: &parser::Input,
        previous_tree: Option<&parser::Tree>,
        options: Option<&parser::Options>,
    ) -> parser::Tree;

    #[wasm_bindgen(method, js_name = parse)]
    pub fn parse_with_string(
        this: &Parser,
        input: &JsString,
        previous_tree: Option<&parser::Tree>,
        options: Option<&parser::Options>,
    ) -> parser::Tree;

    #[wasm_bindgen(method, js_name = getLanguage)]
    pub fn get_language(this: &Parser) -> parser::Language;

    #[wasm_bindgen(method, js_name = setLanguage)]
    pub fn set_language(this: &Parser, language: &parser::Language);

    #[wasm_bindgen(method, js_name = getLogger)]
    pub fn get_logger(this: &Parser) -> parser::Logger;

    #[wasm_bindgen(method, js_name = setLogger)]
    pub fn set_logger(this: &Parser, logger: &parser::Logger);
}
