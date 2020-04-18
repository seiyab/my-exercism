const numberToColor = [
  "black",
  "brown",
  "red",
  "orange",
  "yellow",
  "green",
  "blue",
  "violet",
  "grey",
  "white"
] as const;

type Color = typeof numberToColor[number];

const colorToNumber = numberToColor
  .reduce((acc, color, idx) => ({ ...acc, [color]: idx }), {}) as {[k in Color]: number};

export class ResistorColor {
  private readonly _colors: Color[];

  constructor(colors: Color[]) {
    if (colors.length < 2) {
      throw new Error("At least two colors need to be present");
    }
    this._colors = colors;
  }
  value(): number {
    return this._colors
      .slice(0, 2)
      .map((color) => colorToNumber[color])
      .reduce((acc, digit) => acc*10 + digit, 0);
  };
}
