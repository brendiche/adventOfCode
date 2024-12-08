const FILE_INPUT = `${import.meta.dir}/input`;

const input = await Bun.file(FILE_INPUT).text();
const rows = input.split('\n');
console.log(rows);
const [raceTime, distanceToBeat] = rows.map((r) =>
  parseInt(
    r
      .split(':')[1]
      .trim()
      .split(' ')
      .filter((e) => Number.isInteger(parseInt(e)))
      .join('')
  )
);

let solutions = 0;
for (let i = 0; i <= raceTime; i++) {
  const distanceTraveled = (raceTime - i) * i;
  //   console.log('distanceTraveled: ', distanceTraveled);
  if (distanceTraveled > distanceToBeat) solutions++;
}

// console.log('solutions ====> ', solutions);

console.log(solutions);
