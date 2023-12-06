const FILE_INPUT = `${import.meta.dir}/input-abdou`;

type Card = {
  id: number;
  winningNumbers: number[];
  playedNumbers: number[];
  count: number;
};

async function day4b() {
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
      count: 1,
    });
  }

  let total = 0;

  for (const cardIndex in cards) {
    const card = cards[cardIndex];

    const validNumbers = card.playedNumbers.filter((n) =>
      card.winningNumbers.includes(n)
    );

    if (validNumbers.length) {
      for (let j = 0; j < card.count; j++) {
        for (let i = 1; i <= validNumbers.length; i++) {
          if (parseInt(cardIndex) === 0) {
            console.log(`adding count to card ${parseInt(cardIndex) + i + 1}`);
          }
          cards[parseInt(cardIndex) + i].count++;
        }
      }
    }
  }

  console.log(cards.reduce((total, card) => total + card.count, 0));
}

day4b();
