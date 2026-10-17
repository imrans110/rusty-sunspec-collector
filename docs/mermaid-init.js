// Renders ```mermaid code blocks (the architecture diagram) on the docs site. GitHub
// renders the same blocks in the README itself. Mermaid is loaded from a CDN, pinned and
// integrity-checked, and only on pages that have a diagram.
(function () {
  const blocks = document.querySelectorAll("code.language-mermaid");
  if (blocks.length === 0) {
    return;
  }
  for (const block of blocks) {
    const diagram = document.createElement("pre");
    diagram.className = "mermaid";
    diagram.textContent = block.textContent;
    block.parentElement.replaceWith(diagram);
  }
  const darkThemes = ["coal", "navy", "ayu"];
  const dark = darkThemes.some((theme) => document.documentElement.classList.contains(theme));
  const script = document.createElement("script");
  script.src = "https://cdn.jsdelivr.net/npm/mermaid@11.4.0/dist/mermaid.min.js";
  script.integrity = "sha384-Wm9qzEgq4j1jEnuFK2FxKTlwuhbV2QqtGhcchvjDoKxeJ7WWAW7fysBq+1s6myfX";
  script.crossOrigin = "anonymous";
  script.onload = function () {
    window.mermaid.initialize({ startOnLoad: false, theme: dark ? "dark" : "default" });
    window.mermaid.run();
  };
  document.head.appendChild(script);
})();
