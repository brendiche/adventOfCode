import { Coord, PartNumber, SCAN } from './partNumber';

const FILE_INPUT = `${import.meta.dir}/input`;

const getMatrix = async () => {
  const input = Bun.file(FILE_INPUT);
  const raws = (await input.text()).split('\n');
  return raws.map((raw) => raw.split(''));
};

const checkElementExistInMatrix = (
  element: Coord,
  matrix: string[][]
): boolean => {
  return !(
    element.column < 0 ||
    element.raw < 0 ||
    element.raw >= matrix.length ||
    element.column >= matrix[0].length
  );
};

const isSymbolObject = (element: string) => {
  return !(Number.isInteger(parseInt(element)) || element === '.');
};

const checkAdjacentSymbol = (element: Coord, matrix: string[][]) => {
  let symbolDetected = false;
  SCAN.forEach((delta) => {
    const coordCheckElement: Coord = {
      column: element.column + delta.column,
      raw: element.raw + delta.raw,
    };
    if (checkElementExistInMatrix(coordCheckElement, matrix)) {
      if (
        isSymbolObject(matrix[coordCheckElement.raw][coordCheckElement.column])
      ) {
        symbolDetected = true;
      }
    }
  });
  return symbolDetected;
};

const isPartNumber = (partNumber: PartNumber, matrix: string[][]) => {
  let validPartNumber = false;
  partNumber.getPartials().forEach((partialPartNumber) => {
    if (checkAdjacentSymbol(partialPartNumber.coord, matrix)) {
      validPartNumber = true;
    }
  });
  return validPartNumber;
};

getMatrix().then((matrix) => {
  const candidatePartNumbers: PartNumber[] = [];
  const potentialGears: Coord[] = [];
  matrix.forEach((raw, rIndex) => {
    let partNumber: PartNumber = new PartNumber();
    raw.forEach((element, cIndex) => {
      if (Number.isInteger(parseInt(element))) {
        partNumber.addPartial({
          value: parseInt(element),
          coord: {
            column: cIndex,
            raw: rIndex,
          },
        });
      } else {
        if (!partNumber.isEmpty()) {
          candidatePartNumbers.push(partNumber);
          partNumber = new PartNumber();
        }
        if (element === '*')
          potentialGears.push({
            column: cIndex,
            raw: rIndex,
          });
      }
    });
    if (!partNumber.isEmpty()) candidatePartNumbers.push(partNumber);
  });
  const partNumbers: PartNumber[] = [];
  candidatePartNumbers.forEach((candidatePartNumber) => {
    if (isPartNumber(candidatePartNumber, matrix)) {
      partNumbers.push(candidatePartNumber);
    }
  });

  let answer = 0;
  partNumbers.forEach((partNumber) => {
    answer += partNumber.getValue();
  });
  console.log('answer P1 : ', answer);
  let answerP2 = 0;
  potentialGears.forEach((potentialGear) => {
    const partNumbersId = partNumbers
      .map((partNumber) => {
        return partNumber.isAdjacent(potentialGear) ? partNumber.getId() : null;
      })
      .filter((partNumberId) => partNumberId);

    if (partNumbersId.length === 2) {
      const partnumber1 = partNumbers.find(
        (partNumber) => partNumber.getId() === partNumbersId[0]
      );
      const partnumber2 = partNumbers.find(
        (partNumber) => partNumber.getId() === partNumbersId[1]
      );
      const ratio =
        (partnumber1?.getValue() ?? 1) * (partnumber2?.getValue() ?? 1);
      answerP2 += ratio;
    }
  });
  console.log('answer P2 : ', answerP2);
});
