const FILE_INPUT = `${import.meta.dir}/input`;

type COLORS = 'red' | 'green' | 'blue';

async function processLineByLinePossibleGame(ref: { [key in COLORS]: number }) {
  const input = Bun.file(FILE_INPUT);
  const inputLines = (await input.text()).split('\n');
  let answer = 0;
  inputLines.forEach((line) => {
    let gamePossible = true;
    const [game, rawSets] = line.split(':');
    const gameId = game.trim().split(' ')[1];
    const sets = rawSets.split(';');
    console.log(game, sets);
    console.log('game : ', gameId);
    sets.forEach((set, setInedx) => {
      const candidate: { [key in COLORS]: number } = {
        red: 0,
        green: 0,
        blue: 0,
      };
      const cubesAmountAndColors = set.trim().split(',');
      console.log('set : ', setInedx);
      cubesAmountAndColors.forEach((cubeAmountAndColor) => {
        const [cubeAmount, color] = cubeAmountAndColor.trim().split(' ');
        candidate[color as COLORS] = parseInt(cubeAmount);
      });
      console.log('candidate :', candidate);
      if (
        !(
          candidate.blue <= ref.blue &&
          candidate.red <= ref.red &&
          candidate.green <= ref.green
        )
      ) {
        gamePossible = false;
      }
    });
    if (gamePossible) answer += parseInt(gameId);
  });
  console.log('answer:', answer);
}

processLineByLinePossibleGame({
  blue: 14,
  green: 13,
  red: 12,
});

async function processLineByLinePowerGame() {
  const input = Bun.file(FILE_INPUT);
  const inputLines = (await input.text()).split('\n');
  let answer = 0;
  inputLines.forEach((line) => {
    const [game, rawSets] = line.split(':');
    const gameId = game.trim().split(' ')[1];
    const sets = rawSets.split(';');
    console.log(game, sets);
    console.log('game : ', gameId);
    const candidates: { [key in COLORS]: number }[] = [];
    sets.forEach((set, setInedx) => {
      const candidate: { [key in COLORS]: number } = {
        red: 0,
        green: 0,
        blue: 0,
      };
      const cubesAmountAndColors = set.trim().split(',');
      console.log('set : ', setInedx);
      cubesAmountAndColors.forEach((cubeAmountAndColor) => {
        const [cubeAmount, color] = cubeAmountAndColor.trim().split(' ');
        candidate[color as COLORS] = parseInt(cubeAmount);
      });
      console.log('candidate :', candidate);
      candidates.push(candidate);
    });
    const minimumSetForTheGame: { [key in COLORS]: number } = candidates.reduce(
      (resultSet, candidate) => {
        resultSet.blue = Math.max(resultSet.blue, candidate.blue);
        resultSet.red = Math.max(resultSet.red, candidate.red);
        resultSet.green = Math.max(resultSet.green, candidate.green);
        return resultSet;
      },
      { green: 0, red: 0, blue: 0 }
    );
    const gamePower =
      minimumSetForTheGame.blue *
      minimumSetForTheGame.red *
      minimumSetForTheGame.green;
    answer += gamePower;
  });
  console.log('answer:', answer);
}

processLineByLinePowerGame();
