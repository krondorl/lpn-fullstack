/**
 * @license
 * Attribution-NonCommercial-NoDerivatives 4.0 International (CC BY-NC-ND 4.0)
 * https://creativecommons.org/licenses/by-nc-nd/4.0/
 * Made by Adam Burucs in 2023.
 *
 * Please see LICENSE file in the repo root folder.
 */

const resultContainer = document.querySelector(".result");
const resultSpan = document.querySelector(".result-number");
const summaries = document.querySelectorAll("details");

function closeLPDetails() {
  summaries.forEach((summary) => {
    summary.open = false;
    summary.className = "";
  });
}

document.querySelector("form").addEventListener("submit", (e) => {
  e.preventDefault();
});

document.querySelector("#reset").addEventListener("click", () => {
  closeLPDetails();
  resultContainer.style.display = "none";
  resultSpan.textContent = "";
});

document.querySelector("#submit").addEventListener("click", () => {
  closeLPDetails();
  summaries.forEach((summary) => (summary.open = false));
  const _date = document.querySelector("#date").value;
  if (_date && _date.length === 10) {
    // @todo GET Request
  } else {
    console.log("Wrong date ", _date);
  }
});
