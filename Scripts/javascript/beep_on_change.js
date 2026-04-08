//Options:{
//  text: "Waiting", The text to watch for change
//  typeToObserve: "text", "text" or "class" picks whether to watch for text content changes or class changes
//  watchType: "changeFrom", "changeTo", toggles whether the beep happens when the text changes from or to the specified text
//}
//

((target, options) => {
  // Function to play a beep
  function beep() {
    const ctx = new (window.AudioContext || window.webkitAudioContext)();
    const oscillator = ctx.createOscillator();
    oscillator.type = "sine";
    oscillator.frequency.setValueAtTime(1000, ctx.currentTime); // 1000 Hz
    oscillator.connect(ctx.destination);
    oscillator.start();
    oscillator.stop(ctx.currentTime + 0.2); // 0.2 seconds

    fetch("10.167.214.4/notify");
  }

  const observeText = () => {
    // Observer to watch for changes
    const observer = new MutationObserver(mutations => {
      mutations.forEach(mutation => {
        if (mutation.type === "characterData" || mutation.type === "childList") {
          if (options.watchType === "changeTo" && target.textContent.trim().includes(options.text)) {
            beep();
            console.log("Status changed to:", target.textContent.trim());
            observer.disconnect(); 
          } else if (options.watchType === "changeFrom" && !target.textContent.trim().includes(options.text)) {
            beep();
            console.log("Status changed to:", target.textContent.trim());
            observer.disconnect();
          }
        }
      });
    });

    // Start observing the element
    observer.observe(target, { childList: true, characterData: true, subtree: true });
  }

  const observeClass = () => {
    // Observer to watch for changes
    const observer = new MutationObserver(mutations => {
      mutations.forEach(mutation => {
        if (mutation.type === "attributes" && mutation.attributeName === "class") {
          if (options.watchType === "changeTo" && target.classList.contains(options.text)) {
            beep();
            console.log("Class changed to:", target.className);
            observer.disconnect(); // Stop watching after first change
          } else if (options.watchType === "changeFrom" && !target.classList.contains(options.text)) {
            beep();
            console.log("Class changed to:", target.className);
            observer.disconnect(); // Stop watching after first change
          }
        }
      });
    });

    // Start observing the element
    observer.observe(target, { attributes: true });
  }

  if (options.typeToObserve === "text") {
    // Start observing the text
    observeText();
    console.log("Watching for text change...");
  } else if (options.typeToObserve === "class") {
    // Start observing the class
    observeClass();
    console.log("Watching for class change...");
  }
  else {
    console.error("Invalid type to observe. Use 'text' or 'class'.");
  }
})(temp1, { text: "neutral", typeToObserve: "class", watchType: "changeFrom" });

