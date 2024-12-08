export type Coord = {
  raw: number;
  column: number;
};

export const SCAN: Coord[] = [
  {
    column: -1,
    raw: -1,
  },
  {
    column: 0,
    raw: -1,
  },
  {
    column: 1,
    raw: -1,
  },
  {
    column: -1,
    raw: 0,
  },
  {
    column: 1,
    raw: 0,
  },
  {
    column: -1,
    raw: 1,
  },
  {
    column: 0,
    raw: 1,
  },
  {
    column: 1,
    raw: 1,
  },
];

export class PartNumber {
  private uuid: string;
  private partials: {
    value: number;
    coord: Coord;
  }[];

  constructor() {
    this.uuid = crypto.randomUUID();
    this.partials = [];
  }

  getPartials() {
    return this.partials;
  }

  getId() {
    return this.uuid;
  }

  isEmpty() {
    return this.partials.length === 0;
  }

  addPartial(partial: { value: number; coord: Coord }) {
    this.partials.push(partial);
  }

  getValue() {
    return parseInt(
      this.partials.reduce((value, partial) => {
        return `${value}${partial.value}`;
      }, '')
    );
  }

  isAdjacent(coord: Coord): boolean {
    let isAdjacent = false;
    this.partials.forEach((partial) => {
      SCAN.forEach((delta) => {
        const coordCheckElement: Coord = {
          column: partial.coord.column + delta.column,
          raw: partial.coord.raw + delta.raw,
        };
        if (
          coordCheckElement.column === coord.column &&
          coordCheckElement.raw === coord.raw
        )
          isAdjacent = true;
      });
    });
    return isAdjacent;
  }
}
