// All updates to VeeDoom nodes are broadcast here, forming a diff stream that
// could be consumed by a client that can render to the DOM. An example dom is
// implemented here, but this is suboptimal because the whole file gets bundled
// with the wasm file and imported at runtime. This file is then requested again
// from the main.mjs file, so that it can use `render` and `createElement`.
// TODO: debounce and batch updates to the same element

export function onChange(diff) {
  console.log('I watched a change', diff);
  switch (diff.field) {
    case 'Tag':
      return updateTag(diff);
    case 'Props':
      return updateProps(diff);
    case 'Children':
      return updateChildren(diff);
  }
}

function updateChildren(diff) {
  switch (diff.change) {
    case 'Deleted':
      return removeChild(diff);
    case 'Created':
      return insertChild(diff);
  }
}

function updateTag(diff) {
  const oldEl = document.getElementById(diff.key); 
  if (oldEl) {
    const parentEl = oldEl.parentNode;
    const newEl = createElement(diff.new_node);
    parentEl.replaceChild(newEl, oldEl);
  }
}

// TODO: properly update/remove props
function updateProps(diff) {
  const el = document.getElementById(diff.key); 
  if (el) {
    for (const key in diff.new_node.props) {
      const val = diff.new_node.props[key];
      el.setAttribute(key, val);
    }

    // This should obviously be done differently but for demo purposes it's ok
    if (diff.new_node.props.hasOwnProperty('text')) {
      el.childNodes.item(0).textContent = diff.new_node.props.text;
    }
  }
}

function removeChild(diff) {
  const el = document.getElementById(diff.key); 
  if (el) {
    el.remove();
  }
}

function insertChild(diff) {
  const parentEl = document.getElementById(diff.key); 
  if (parentEl) {
    const newEl = createElement(diff.new_node);
    if (parentEl.childNodes.length >= diff.new_pos + 1) {
      const sibling = parentEl.childNodes[diff.new_pos + 1];
      if (sibling) {
        parentEl.insertBefore(newEl, sibling);
        return;
      } else {
        // there's a problem
      }
    }
    parentEl.appendChild(newEl);
  }
}

export function createElement(node) {
  const fragment = document.createDocumentFragment();
  const el = document.createElement(node.tag);
  for (const key in node.props) {
    const val = node.props[key];
    el.setAttribute(key, val);
  }

  el.setAttribute('id', node.key);

  if (node.props.hasOwnProperty('text')) {
    const text = document.createTextNode(node.props.text);
    el.appendChild(text);
  }

  if (node.children.length > 0) {
    node.children.forEach(childNode => {
      const childEl = createElement(childNode);
      el.append(childEl);
    });
  }

  fragment.appendChild(el);
  return fragment;
}

export function render(node, rootId) {
  const root = document.getElementById(rootId);
  if (root) {
    const el = createElement(node);
    root.appendChild(el); 
  }
}
