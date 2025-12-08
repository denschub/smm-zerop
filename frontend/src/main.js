import "./main.css";

document.body.classList.add("js");

function initClickCopy() {
  for (const container of document.querySelectorAll(".clickcopy-container")) {
    const content = container.querySelector(".clickcopy-content");
    const popover = container.querySelector(".popover");

    container.addEventListener("click", () => {
      navigator.clipboard.writeText(content.innerText.trim());

      popover.style.display = "";
      window.setTimeout(() => {
        popover.style.display = "none";
      }, 2000);
    });
  }
}

function initFancySelects() {
  const offsets = [-1, 1];
  for (const container of document.querySelectorAll("div.fancy-select")) {
    const select = container.querySelector("select");
    const maxOptions = select.options.length - 1;

    const [prevButton, nextButton] = [...container.querySelectorAll("button")].map((button, i) => {
      button.addEventListener("click", () => {
        select.selectedIndex = Math.min(Math.max(select.selectedIndex + offsets[i], 0), maxOptions);
        select.dispatchEvent(new Event("change"));
      });

      return button;
    });

    const updateButtonStates = () => {
      prevButton.disabled = select.selectedIndex == 0;
      nextButton.disabled = select.selectedIndex == maxOptions;
    };
    select.addEventListener("change", updateButtonStates);
    updateButtonStates();
  }
}

function initThumbnailLoaders() {
  for (const loader of document.querySelectorAll(".thumbnail-loader")) {
    const img = loader.querySelector(".image-container img");
    const spinnerLoading = loader.querySelector(".spinner-loading");
    const spinnerError = loader.querySelector(".spinner-error");

    img.src = img.dataset.src;
    img.addEventListener("load", async () => {
      await img.decode();
      img.style.visibility = "";
      spinnerLoading.style.display = "none";
    });

    img.addEventListener("error", () => {
      spinnerLoading.style.display = "none";
      spinnerError.style.display = "block";
    });
  }
}

const currentUrl = new URL(window.location);
if (currentUrl.pathname == "/smm2/random_level/") {
  document.addEventListener("DOMContentLoaded", () => {
    initClickCopy();
    initFancySelects();
    initThumbnailLoaders();
  });
}
