// TODO : get the argument and launch the code for the day

import inputD1 from './input.json';

const LETTER_DIGIT_DICO = {
  one: 1,
  two: 2,
  three: 3,
  four: 4,
  five: 5,
  six: 6,
  seven: 7,
  eight: 8,
  nine: 9,
};

type STRING_DIGIT = keyof typeof LETTER_DIGIT_DICO;

const answer = inputD1
  .map((row) => {
    const chars = row.split('');
    const firstDigit = chars.find((char) => Number.isInteger(parseInt(char)));
    const lastDigit = chars.findLast((char) =>
      Number.isInteger(parseInt(char))
    );
    return `${firstDigit}${lastDigit}`;
  })
  .reduce((sum, inputNumber) => (sum += parseInt(inputNumber)), 0);

console.log('d1 p1 code :', answer);

const replaceDigitStringByNumber = (rowLowerCase: string) => {
  let finalResponse = rowLowerCase;
  const stringsDigit = Object.keys(LETTER_DIGIT_DICO) as STRING_DIGIT[];

  stringsDigit.forEach((stringDigit: STRING_DIGIT) => {
    finalResponse = finalResponse.replaceAll(
      stringDigit,
      `${stringDigit}${LETTER_DIGIT_DICO[stringDigit]}${stringDigit}`
    );
  });
  return finalResponse;
};

const answerP2 = inputD1
  .map((row) => {
    const chars = replaceDigitStringByNumber(row.toLocaleLowerCase()).split('');
    const firstDigit = chars.find((char) => Number.isInteger(parseInt(char)));
    const lastDigit = chars.findLast((char) =>
      Number.isInteger(parseInt(char))
    );
    return `${firstDigit}${lastDigit}`;
  })
  .reduce((sum, inputNumber) => (sum += parseInt(inputNumber)), 0);

console.log('d1 p2 code :', answerP2);
