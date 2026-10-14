// The site's only script: the bits of interactivity the prerendered HTML needs.
// Class names toggled here must stay literal so Tailwind keeps them in the CSS.

// Mobile menu
for (const button of document.querySelectorAll("[data-menu-toggle]")) {
  const menu = document.getElementById(button.getAttribute("aria-controls"));
  button.addEventListener("click", () => {
    const open = button.getAttribute("aria-expanded") !== "true";
    button.setAttribute("aria-expanded", String(open));
    menu.classList.toggle("max-h-0", !open);
    menu.classList.toggle("max-h-screen", open);
    button.querySelector('[data-menu-icon="open"]').classList.toggle("hidden", open);
    button.querySelector('[data-menu-icon="close"]').classList.toggle("hidden", !open);
  });
}

// Announcement modal
const modal = document.querySelector("[data-modal]");
if (modal) {
  const close = () => modal.classList.add("hidden");
  for (const button of document.querySelectorAll("[data-modal-open]")) {
    button.addEventListener("click", () => modal.classList.remove("hidden"));
  }
  modal.addEventListener("click", (event) => {
    if (!event.target.closest("[data-modal-panel]") || event.target.closest("[data-modal-close]")) {
      close();
    }
  });
  document.addEventListener("keydown", (event) => {
    if (event.key === "Escape") close();
  });
}

// Gallery: click an image to expand it to full width, again to shrink it back
const galleryItems = document.querySelectorAll("[data-gallery-item]");
for (const item of galleryItems) {
  item.addEventListener("click", () => {
    const expand = !item.classList.contains("col-span-full");
    for (const other of galleryItems) {
      other.classList.remove("col-span-full");
      other.classList.add("hover:scale-105");
    }
    if (expand) {
      item.classList.add("col-span-full");
      item.classList.remove("hover:scale-105");
      requestAnimationFrame(() => item.scrollIntoView({ behavior: "smooth", block: "center" }));
    }
  });
}
