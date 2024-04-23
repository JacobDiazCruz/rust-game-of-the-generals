# Rust Game of the Generals

### Board Layout
```css
.board {
  display: grid;
  grid-template-columns: repeat(9, 40px); /* Adjust width as needed */
  grid-auto-rows: 40px; /* Adjust height as needed */
  gap: 2px; /* Adjust gap between cells */
}

.cell {
  display: flex;
  justify-content: center;
  align-items: center;
  border: 1px solid black;
  width: 40px; /* Adjust width as needed */
  height: 40px; /* Adjust height as needed */
}

```html
<div class="board">
  <div class="cell">11</div>
  <div class="cell">12</div>
  <!-- Add more cells here -->
</div>

### Cell Numbering
Each cell is represented by two digits: [row][column].
The first digit indicates the row number.
The second digit indicates the column number.