const FILE_INPUT = `${import.meta.dir}/test`;

type Card = {
  id: number;
  winnigs: number[];
  candidates: number[];
};

const getCards = async () => {
  const cards: Card[] = [];
  const input = Bun.file(FILE_INPUT);
  const cardsInput = (await input.text()).split('\n');
  cardsInput.forEach((card) => {
    const [cardId, mixedNumbers] = card.split(':');
    const [winnigsNumbers, candidatesNumbers] = mixedNumbers.split('|');
    const cardToSave: Card = {
      id: parseInt(cardId.split(' ')[1]),
      winnigs: winnigsNumbers
        .trim()
        .split(' ')
        .map((n) => parseInt(n))
        .filter((n) => Number.isInteger(n)),
      candidates: candidatesNumbers
        .trim()
        .split(' ')
        .map((n) => parseInt(n))
        .filter((n) => Number.isInteger(n)),
    };
    cards.push(cardToSave);
  });
  return cards;
};

const calculateCardPoint = (card: Card) => {};

getCards().then((cards) => {
  console.log(cards);
  cards.forEach((card) => {
    calculateCardPoint(card);
  });
});
