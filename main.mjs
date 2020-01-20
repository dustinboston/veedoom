import init, { v, update } from './pkg/veedoom.js';
import { render } from './dom.js';

async function main() {
  await init('/pkg/veedoom_bg.wasm');

  let example = v("div", {
    key: "foo",
    text: "Hello... ",
    style: "font-size: 20px"
  }, []);

  // Initial render which sets up the root node
  render(example, "root");

  // Updates...
  setTimeout(() => {
    example = update(
      example,
      { ...example, tag: "code" }
    );
  }, 1000);

  setTimeout(() => {
    example = update(
      example,
      { ...example, props: {
        text: "HELLO... " 
      }}
    );
  }, 2000);

  setTimeout(() => {
    example = update(
      example,
      { ...example, children: [
        v("strong", {
          key: "bar",
          text: "VEEDOOM!",
          style: "color: red"
        }, [])
      ]}
    );
  }, 3000);

}
main()

// TODO: make demo interactive through a "console game", get it...
window.lookAround = () => {
  window.examinePane = () => {
    window.runCommand = (command) => {
      setTimeout(() => {
        console.error('Your command resulted in a catastrophic failure which caused time to fold into itself. You are infinite. And yet, you still crave a good cup of coffee.');
      }, 1000);
      return `Executing command: '${command}'...`;
    };
    return 'As you peer through the pane you see a complex set of mirrors. Every movent you make appears to be refracted here and there until finally they are echoed back as the simple commands which conceived them. Your perspective has changed.';
  };
  return 'You see a window with several panes. One appears different than the rest, as though recently cleaned.'; 
};

