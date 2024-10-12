const { invoke } = window.__TAURI__.core;

let personNameInput;
let personAgeInput;
let personAddressInput;
let peopleTable;

async function save_person() {
    await invoke("save_person", {
        person: {
            name: personNameInput.value,
            age: parseInt(personAgeInput.value),
            address: personAddressInput.value,
        }
    });
}

async function load_people() {
    const people = await invoke("load_people");
    peopleTable.innerHTML = ""
    people.forEach(person => {
        const row = document.createElement("tr");
        row.innerHTML = `
        <td>${person.name}</td>
        <td>${person.age}</td>
        <td>${person.address}</td>
        `;
        peopleTable.appendChild(row);
    });
}

window.addEventListener("DOMContentLoaded", () => {
  personNameInput = document.querySelector("#name");
  personAgeInput = document.querySelector("#age");
  personAddressInput = document.querySelector("#address");
  // People table #people-table tbody
  peopleTable = document.querySelector("#people-table tbody");
  document.querySelector("#person-form").addEventListener("submit", async (e) => {
      e.preventDefault();
      await save_person();
      await load_people();
  });
});

load_people();