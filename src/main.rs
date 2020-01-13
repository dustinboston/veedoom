use veedoom::*;
pub fn main() {
    println!("nothing to see here, run some tests");

    // CARE BEAR STARE!
    let mut tree = n(
        "ul",
        vec![],
        vec![
            n("li", vec![("text", "C")], vec![]),
            n("li", vec![("text", "A")], vec![]),
            n("li", vec![("text", "R")], vec![]),
            n("li", vec![("text", "E")], vec![]),
        ],
    );

    let mut children_change_ex = tree.clone();
    children_change_ex.children.remove(0);
    children_change_ex
        .children
        .insert(0, n("li", vec![("text", "B")], vec![]));
    children_change_ex
        .children
        .insert(1, n("li", vec![("text", "E")], vec![]));
    children_change_ex.children.remove(4);

    tree.update(children_change_ex);
}

#[cfg(test)]
mod tests {
    use veedoom::*;

    #[test]
    fn should_detect_tag_change() {
        let mut tree = n("blink", vec![], vec![]);
        let mut tag_change_ex = tree.clone();
        tag_change_ex.tag = "marquee".to_string();

        tree.update(tag_change_ex);

        assert_eq!(tree.tag, "marquee".to_string());
    }

    #[test]
    fn should_detect_props_change() {
        let mut tree = n("ul", vec![("class", "list")], vec![]);

        let mut props_change_ex = tree.clone();
        *props_change_ex.props.get_mut("class").unwrap() = "nums".to_string();
        props_change_ex
            .props
            .insert("title".to_string(), "updated".to_string());

        tree.update(props_change_ex);

        assert_eq!(tree.props.get("class").unwrap(), "nums");
        assert_eq!(tree.props.contains_key("title"), true);
        assert_eq!(tree.props.get("title").unwrap(), "updated");
    }

    #[test]
    fn should_detect_children_change() {
        let mut tree = n(
            "ul",
            vec![],
            vec![
                n("li", vec![("text", "item 1")], vec![]),
                n("li", vec![("text", "item 2")], vec![]),
            ],
        );
        let mut children_change_ex = tree.clone();
        children_change_ex.children.remove(0);

        assert_eq!(tree.children.len(), 2);
        tree.update(children_change_ex);
        assert_eq!(tree.children.len(), 1);
    }

    #[test]
    fn should_recurse_children() {
        let mut tree = n(
            "ul",
            vec![("key", "foo")],
            vec![
                n("li", vec![("key", "bar")], vec![
                    n("a", vec![("key", "baz")], vec![])
                ])
            ],
        );

        let deep_changes = n(
            "ul",
            vec![("key", "foo")],
            vec![
                n("li", vec![("key", "bar")], vec![
                    n("span", vec![("key", "baz")], vec![])
                ])
            ],
        );

        assert_eq!(tree.children[0].children[0].tag, "a");
        tree.update(deep_changes);
        assert_eq!(tree.children[0].children[0].tag, "span");
    }
}
