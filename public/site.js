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

// Home page video: pause/play button, and stay paused for visitors who
// prefer reduced motion
for (const button of document.querySelectorAll("[data-video-toggle]")) {
  const video = document.getElementById(button.getAttribute("aria-controls"));
  const show = (playing) => {
    button.setAttribute("aria-label", playing ? button.dataset.labelPause : button.dataset.labelPlay);
    button.querySelector('[data-video-icon="pause"]').classList.toggle("hidden", !playing);
    button.querySelector('[data-video-icon="play"]').classList.toggle("hidden", playing);
  };
  if (matchMedia("(prefers-reduced-motion: reduce)").matches) {
    video.removeAttribute("autoplay");
    video.pause();
  }
  video.addEventListener("play", () => show(true));
  video.addEventListener("pause", () => show(false));
  show(!video.paused || video.autoplay);
  button.addEventListener("click", () => (video.paused ? video.play() : video.pause()));
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

// Christmas menu poster: show it full size over the page instead of leaving
// it for the bare image. The native dialog handles Escape and keeps focus
// inside; a click anywhere but the poster closes it too.
const lightbox = document.querySelector("[data-lightbox]");
if (lightbox) {
  for (const link of document.querySelectorAll("[data-lightbox-open]")) {
    link.addEventListener("click", (event) => {
      event.preventDefault();
      lightbox.showModal();
      document.documentElement.style.overflow = "hidden";
    });
  }
  lightbox.addEventListener("click", (event) => {
    if (event.target.closest("[data-lightbox-close]") || !event.target.closest("img")) {
      lightbox.close();
    }
  });
  lightbox.addEventListener("close", () => {
    document.documentElement.style.overflow = "";
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

// 404 page: rescue links with stray characters stuck to a real page name,
// like "legopal.fr/reservation🙏" shared on social media
const notFound = document.querySelector("[data-not-found]");
if (notFound) {
  const path = location.pathname.toLowerCase();
  const page = notFound.dataset.notFound
    .split(" ")
    .find((name) => new RegExp(`^/${name}(?![a-z0-9])`).test(path));
  if (page) location.replace(`/${page}${location.search}${location.hash}`);
}
