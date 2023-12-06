const FILE_INPUT = `${import.meta.dir}/input-abdou`;

type Card = {
  id: number;
  winningNumbers: number[];
  playedNumbers: number[];
};

async function day4a() {
  const input = Bun.file(FILE_INPUT);

  const lines = (await input.text()).split('\n');

  const cards: Array<Card> = [];
  for (const line of lines) {
    const [cardNumber, numbers] = line.split(': ');
    const [winningNumbers, playedNumbers] = numbers.split(' | ');
    const id = parseInt(cardNumber.split(' ')[1]);

    cards.push({
      id,
      winningNumbers: winningNumbers
        .split(' ')
        .map((n) => parseInt(n))
        .filter((n) => Number.isInteger(n)),
      playedNumbers: playedNumbers
        .split(' ')
        .map((n) => parseInt(n))
        .filter((n) => Number.isInteger(n)),
    });
  }

  let total = 0;

  for (const card of cards) {
    const validNumbers = card.playedNumbers.filter((n) =>
      card.winningNumbers.includes(n)
    );

    if (validNumbers.length) {
      total += Math.pow(2, validNumbers.length - 1);
    }
  }

  console.log(total);
}

day4a();
