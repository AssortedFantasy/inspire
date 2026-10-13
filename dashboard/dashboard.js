"use strict";

const explorer = document.querySelector("#explorer");
const jump = document.querySelector("#jump");
const palette = [28, 45, 165, 205, 95, 260, 325, 185, 38, 135, 218, 285, 10, 65, 240];
const percent = value => value > 0 && value < 0.001
  ? "<0.1%"
  : `${Number((value * 100).toFixed(1))}%`;
const label = node => node.name.replaceAll("-", " ");
const randomSeed = () => {
  const words = crypto.getRandomValues(new Uint32Array(2));
  return ((BigInt(words[0]) << 32n) | BigInt(words[1])).toString();
};
let tree;
let path = [];
let stream;
let observer;

function element(tag, className, text) {
  const result = document.createElement(tag);
  if (className) result.className = className;
  if (text !== undefined) result.textContent = text;
  return result;
}

async function getJSON(url, signal) {
  const response = await fetch(url, { signal });
  const result = await response.json();
  if (!response.ok) throw new Error(result.error || `Request failed (${response.status})`);
  return result;
}

function tint(target, node) {
  target.style.setProperty("--tint", `hsl(${node.hue} 30% 93%)`);
  target.style.setProperty("--hover", `hsl(${node.hue} 32% 85%)`);
}

function localShare(parent, child) {
  return child.weight / parent.children.reduce((sum, node) => sum + node.weight, 0);
}

function describe(node) {
  if (node.kind === "branch") return `${node.children.length} children`;
  return node.kind === "dataset" ? `${node.count.toLocaleString()} records` : "Generator";
}

function previewNode(preview, node) {
  preview.replaceChildren();
  preview.append(element("p", "preview-title", `${node.path} · ${describe(node)} · ${percent(node.probability)} overall`));
  if (node.children.length) {
    const strip = element("div", "preview-strip");
    for (const child of node.children.filter(child => child.weight > 0)) {
      const tile = element("span", "preview-node", label(child));
      tile.style.flex = `0 0 ${localShare(node, child) * 100}%`;
      tint(tile, child);
      strip.append(tile);
    }
    preview.append(strip);
    const disabled = node.children.filter(child => child.weight === 0);
    if (disabled.length) preview.append(element("p", "", `Zero weight: ${disabled.map(label).join(", ")}`));
  }
  preview.hidden = false;
}

function level(parent, depth) {
  const section = element("section", "level");
  section.id = `level-${depth}`;
  const heading = element("div", "level-heading");
  const title = element("h2", "", parent.path || "All families");
  title.tabIndex = -1;
  heading.append(title, element("p", "", `${parent.children.length} children · ${percent(parent.probability)} overall`));
  const strip = element("div", "strip");
  const stripScroll = element("div", "strip-scroll");
  strip.style.minWidth = `${parent.children.filter(child => child.weight > 0).length * 78}px`;
  stripScroll.append(strip);
  const bridge = element("div", "bridge");
  const preview = element("div", "preview");
  preview.hidden = true;
  preview.setAttribute("aria-hidden", "true");
  const disabled = element("div", "disabled-nodes");
  const selected = path[depth];
  for (const child of parent.children) {
    const button = element("button", "node");
    button.type = "button";
    button.dataset.node = child.id;
    button.style.flex = `0 0 ${localShare(parent, child) * 100}%`;
    tint(button, child);
    const share = percent(localShare(parent, child));
    button.title = `${child.path}\nWeight ${child.weight} · ${share} of this branch\n${describe(child)} · ${percent(child.probability)} overall`;
    button.setAttribute("aria-label", `${label(child)}, ${share} of this branch, weight ${child.weight}, ${describe(child)}`);
    button.setAttribute("aria-pressed", String(selected === child));
    if (child.kind === "branch") button.setAttribute("aria-expanded", String(selected === child));
    const content = element("span", "node-content");
    content.append(element("span", "node-name", label(child)), element("span", "node-share", share), element("span", "node-kind", child.kind === "branch" ? "Branch" : child.kind === "dataset" ? "Dataset" : "Generator"));
    button.append(content);
    button.addEventListener("pointerenter", () => previewNode(preview, child));
    button.addEventListener("pointerleave", () => { preview.hidden = true; });
    button.addEventListener("focus", () => previewNode(preview, child));
    button.addEventListener("blur", () => { preview.hidden = true; });
    button.addEventListener("click", () => {
      path = [...path.slice(0, depth), child];
      render(depth + 1);
    });
    if (child.weight > 0) strip.append(button);
    else {
      button.className = "";
      button.textContent = label(child);
      disabled.append(button);
    }
  }
  if (selected) {
    const svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
    svg.classList.add("connector");
    svg.setAttribute("viewBox", "0 0 1000 105");
    svg.setAttribute("preserveAspectRatio", "none");
    svg.setAttribute("aria-hidden", "true");
    const polygon = document.createElementNS(svg.namespaceURI, "polygon");
    const updateConnector = () => {
      const tile = strip.querySelector(`[data-node="${selected.id}"]`);
      const viewport = stripScroll.getBoundingClientRect();
      const rect = tile?.getBoundingClientRect();
      const clamp = value => Math.max(0, Math.min(1000, value));
      const left = rect ? clamp((rect.left - viewport.left) / viewport.width * 1000) : 0;
      const right = rect ? clamp((rect.right - viewport.left) / viewport.width * 1000) : 0;
      polygon.setAttribute("points", `${left},0 ${right},0 1000,105 0,105`);
    };
    stripScroll.addEventListener("scroll", updateConnector);
    const resize = new ResizeObserver(updateConnector);
    section.resizeObserver = resize;
    resize.observe(stripScroll);
    requestAnimationFrame(updateConnector);
    svg.append(polygon);
    bridge.append(svg);
  }
  bridge.append(preview);
  section.append(heading, stripScroll);
  if (disabled.childElementCount) {
    disabled.prepend(element("span", "", "Zero weight (still inspectable):"));
    section.append(disabled);
  }
  section.append(bridge);
  return section;
}

function cancelStream() {
  observer?.disconnect();
  stream?.controller.abort();
  stream = undefined;
}

function leaf(node) {
  const section = element("section", "leaf");
  section.id = `level-${path.length}`;
  const heading = element("div", "leaf-heading");
  const title = element("h2", "", node.path);
  title.tabIndex = -1;
  title.append(element("span", "kind", node.kind === "dataset" ? "Dataset" : "Generator"));
  const probability = element("p", "probability", "Total weighted probability: ");
  probability.append(element("strong", "", percent(node.probability)));
  heading.append(title, probability);
  const facts = element("div", "facts");
  facts.append(element("p", "", node.probability > 0 ? `About 1 in ${(1 / node.probability).toLocaleString(undefined, { maximumFractionDigits: 1 })} root samples` : "Zero probability from the root; available by explicit selection"));
  const factors = path.map((child, index) => percent(localShare(index ? path[index - 1] : tree, child)));
  facts.append(element("p", "", `Path probability: ${factors.join(" × ")}`));
  const toolbar = element("div", "toolbar");
  const restart = element("button", "", node.kind === "dataset" ? "Reshuffle" : "New samples");
  restart.type = "button";
  const progress = element("p", "progress");
  progress.setAttribute("role", "status");
  toolbar.append(restart, progress);
  const content = element("div", "outputs");
  const loadArea = element("div", "load-area");
  const load = element("button", "", "Load more");
  load.type = "button";
  const status = element("p", "");
  status.setAttribute("role", "status");
  loadArea.append(load, status);
  section.append(heading, facts, toolbar, content, loadArea);

  function start() {
    cancelStream();
    content.replaceChildren();
    const current = { node, seed: randomSeed(), offset: 0, loading: false, done: false, failed: false, controller: new AbortController() };
    stream = current;
    observer = new IntersectionObserver(entries => {
      if (entries.some(entry => entry.isIntersecting) && !current.failed) loadBatch(current);
    }, { rootMargin: "300px" });
    observer.observe(loadArea);
    return loadBatch(current);
  }

  async function loadBatch(current) {
    if (current !== stream || current.loading || current.done) return;
    current.loading = true;
    current.failed = false;
    load.disabled = true;
    load.hidden = false;
    status.className = "";
    status.textContent = "Loading outputs…";
    try {
      const query = new URLSearchParams({ node: node.id, seed: current.seed, offset: current.offset });
      const batch = await getJSON(`/api/outputs?${query}`, current.controller.signal);
      if (current !== stream) return;
      for (const [index, item] of batch.items.entries()) {
        const article = element("article", "output");
        const meta = element("div", "output-meta");
        meta.append(element("span", "", `#${current.offset + index + 1}`));
        meta.append(element("span", "", item.seed ? `Seed ${item.seed}` : `Record ${item.record} of ${node.count}`));
        const copy = element("button", "", "Copy output");
        copy.type = "button";
        copy.addEventListener("click", () => copyText(copy, item.text));
        meta.append(copy);
        if (item.seed) {
          const command = element("button", "", "Copy CLI command");
          command.type = "button";
          command.addEventListener("click", () => copyText(command, `inspire --family ${node.path} --seed ${item.seed}`));
          meta.append(command);
        }
        article.append(meta, element("pre", "", item.text));
        content.append(article);
      }
      current.offset = batch.next;
      current.done = batch.done;
      progress.textContent = node.kind === "dataset"
        ? `${current.offset.toLocaleString()} / ${node.count.toLocaleString()} records · shuffled without repeats`
        : `${current.offset.toLocaleString()} outputs · scroll for more`;
      status.textContent = current.done ? "All records shown. Reshuffle to start again." : "Scroll for more outputs.";
      load.hidden = current.done;
      if (current.done) observer.disconnect();
    } catch (error) {
      if (error.name === "AbortError" || current !== stream) return;
      current.failed = true;
      status.className = "error";
      status.textContent = error.message;
      load.textContent = "Retry";
    } finally {
      if (current === stream) {
        current.loading = false;
        load.disabled = false;
        if (!current.failed) {
          load.textContent = "Load more";
          // Re-observe after appending: short outputs can leave the sentinel in view.
          if (!current.done) { observer.unobserve(loadArea); observer.observe(loadArea); }
        }
      }
    }
  }
  load.addEventListener("click", () => loadBatch(stream));
  restart.addEventListener("click", () => {
    start().then(() => { if (section.isConnected) section.scrollIntoView({ block: "start" }); });
  });
  // The section must be in the document before observing its scroll sentinel.
  section.start = start;
  return section;
}

async function copyText(button, text) {
  const original = button.textContent;
  try {
    await navigator.clipboard.writeText(text);
    button.textContent = "Copied";
  } catch {
    button.textContent = "Copy unavailable";
  }
  setTimeout(() => { if (button.isConnected) button.textContent = original; }, 1800);
}

function render(scrollDepth) {
  cancelStream();
  explorer.querySelectorAll(".level").forEach(section => section.resizeObserver?.disconnect());
  explorer.replaceChildren(level(tree, 0));
  for (const [index, node] of path.entries()) {
    explorer.append(node.kind === "branch" ? level(node, index + 1) : leaf(node));
  }
  jump.value = path.at(-1)?.kind !== "branch" ? String(path.at(-1)?.id ?? "") : "";
  if (scrollDepth !== undefined) {
    const target = document.querySelector(`#level-${scrollDepth}`);
    target.querySelector("h2").focus({ preventScroll: true });
    target.scrollIntoView({ block: "start" });
  }
  const outputSection = explorer.querySelector(".leaf");
  outputSection?.start().then(() => {
    if (scrollDepth === path.length && outputSection.isConnected) outputSection.scrollIntoView({ block: "start" });
  });
}

document.querySelector("#reset").addEventListener("click", () => { path = []; render(0); });
jump.addEventListener("change", () => {
  if (!jump.value) return;
  function find(node, ancestors) {
    if (String(node.id) === jump.value) return ancestors;
    for (const child of node.children) {
      const result = find(child, [...ancestors, child]);
      if (result) return result;
    }
  }
  path = find(tree, []);
  render(path.length);
});

async function initialize() {
  try {
    tree = await getJSON("/api/tree");
    function prepare(node, hue) {
      node.hue = hue;
      for (const child of node.children) prepare(child, hue);
      if (node.kind !== "branch") {
        const option = element("option", "", `${node.path} (${node.kind})`);
        option.value = node.id;
        jump.append(option);
      }
    }
    tree.children.forEach((node, index) => prepare(node, palette[index % palette.length]));
    jump.disabled = false;
    render();
  } catch (error) {
    const message = element("p", "error", `Cannot load tree: ${error.message}`);
    message.setAttribute("role", "alert");
    const retry = element("button", "", "Retry");
    retry.type = "button";
    retry.addEventListener("click", initialize);
    explorer.replaceChildren(message, retry);
  }
}
initialize();
