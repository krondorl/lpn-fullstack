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

const lpn = document.querySelector(".lpn");
const role = document.querySelector(".role");
const positive = document.querySelector(".positive");
const negative = document.querySelector(".negative");

const apiUrl = "http://localhost:8080/api/lpn-calc";

function closeLPDetails() {
  lpn.textContent = "";
  role.textContent = "";
  positive.textContent = "";
  negative.textContent = "";
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
  const _date = document.querySelector("#date").value;
  if (_date && _date.length === 10) {
    fetch(`${apiUrl}/${_date}`)
      .then((lpnDataResponse) => {
        lpnDataResponse.json().then((lpnData) => {
          lpn.textContent = `Life path number: ${lpnData?.lpn}`;
          role.textContent = `Role ${lpnData?.role}`;
          positive.textContent = `Positive traits: ${lpnData?.positive}`;
          negative.textContent = `Negative traits: ${lpnData?.negative}`;
        });
      })
      .catch((err) => {
        console.error("Error during fetch: ", err);
      });
  } else {
    console.log("Wrong date ", _date);
  }
});
