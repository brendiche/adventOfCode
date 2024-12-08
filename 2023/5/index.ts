const FILE_INPUT = `${import.meta.dir}/input`;

type Rule = {
  sourceRangeStart: number;
  targetRangeStart: number;
  rangeLenght: number;
};

const input = Bun.file(FILE_INPUT);
const rows = (await input.text()).split('\n\n');
console.log(rows);

const seeds = rows
  .shift()
  ?.split(':')[1]
  .trim()
  .split(' ')
  .map((s) => parseInt(s));
console.log(seeds);

const maps: { [key: string]: Rule[] } = {};

rows.forEach((row) => {
  maps[row.split(' ')[0]] = row
    .split('\n')
    .slice(1, row.length - 1)
    .map((ruleNumbers) => {
      const [targetRangeStart, sourceRangeStart, rangeLenght] = ruleNumbers
        .trim()
        .split(' ')
        .map((n) => parseInt(n));
      return {
        rangeLenght,
        targetRangeStart,
        sourceRangeStart,
      };
    });
});

console.log(maps);

const path = [
  'seed',
  'soil',
  'fertilizer',
  'water',
  'light',
  'temperature',
  'humidity',
  'location',
];
const finalPlan: number[] = [];
seeds?.forEach((seedRange, idx) => {
  if (idx % 2 === 1) {
    // do sub range by path
    // for (let seed = seeds[idx - 1]; seed < seeds[idx - 1] + seedRange; seed++) {
    let rangeCandidates: {
      [key: string]: { seedRangeStart: number; seedRange: number }[];
    } = {
      seed: [
        {
          seedRangeStart: seeds[idx - 1],
          seedRange,
        },
      ],
    };

    for (let pathIndex = 0; pathIndex < path.length - 1; pathIndex++) {
      const source =
        pathIndex === 0
          ? 'seed'
          : `${path[pathIndex - 1]}-to-${path[pathIndex]}`;
      const currentMapName = `${path[pathIndex]}-to-${
        path[(pathIndex as number) + 1]
      }`;
      console.log('looking at : ', source);
      console.log('in rules : ', currentMapName);

      const rulesCandidates = maps[currentMapName];
      // check if rule over lap candidate range;
      rangeCandidates[currentMapName] = [];
      rangeCandidates[source].forEach((rangeCandidate) => {
        // whitch rule overlap the current range
        const rules = rulesCandidates.filter(
          (r) =>
            Math.min(
              rangeCandidate.seedRangeStart + rangeCandidate.seedRange,
              r.sourceRangeStart + r.rangeLenght
            ) -
              Math.max(rangeCandidate.seedRangeStart, r.sourceRangeStart) >
            0
        );
        // in this i miss the part range source that is not mapped
        if (rules.length) {
          console.log('find rule for : ', currentMapName);
          console.log('rules ', rules);
          // apply the mapping to the candidate
          rules.forEach((rule) => {
            const seedRangeStart =
              Math.max(rangeCandidate.seedRangeStart, rule.sourceRangeStart) +
              (rule.targetRangeStart - rule.sourceRangeStart);

            rangeCandidates[currentMapName].push({
              seedRangeStart,
              seedRange:
                Math.min(
                  rangeCandidate.seedRangeStart + rangeCandidate.seedRange,
                  rule.sourceRangeStart + rule.rangeLenght
                ) +
                (rule.targetRangeStart - rule.sourceRangeStart) -
                seedRangeStart,
            });
          });
          // unmap part range
          const allRulesStartRange = Math.min(
            ...rules.map((r) => r.sourceRangeStart)
          );
          const allRulesRange = rules.reduce((sum, rule) => {
            sum += rule.rangeLenght;
            return sum;
          }, 0);

          if (allRulesStartRange > rangeCandidate.seedRangeStart) {
            console.log('not mapped rule at the start');
            // map 1:1 from rangeCandidate.seedRangeStart to allRulesStartRange
            rangeCandidates[currentMapName].push({
              seedRangeStart: rangeCandidate.seedRangeStart,
              seedRange: allRulesStartRange - rangeCandidate.seedRangeStart,
            });
          }

          if (
            allRulesStartRange + allRulesRange <
            rangeCandidate.seedRangeStart + rangeCandidate.seedRange
          ) {
            console.log('not mapped rule at the end');
            // map 1:1 from allRulesStartRange+allRulesRange to rangeCandidate.seedRangeStart+rangeCandidate.seedRange
            rangeCandidates[currentMapName].push({
              seedRangeStart: allRulesStartRange + allRulesRange,
              seedRange:
                rangeCandidate.seedRangeStart +
                rangeCandidate.seedRange -
                (allRulesStartRange + allRulesRange),
            });
          }

          console.log('sub candidate', rangeCandidates);
        } else {
          rangeCandidates[currentMapName].push(rangeCandidate);
          console.log(
            'don t find any rules \n sub candidate',
            rangeCandidates[currentMapName]
          );
        }
      });
    }

    console.log(rangeCandidates);
    const min = Math.min(
      ...rangeCandidates['humidity-to-location'].map((c) => c.seedRangeStart)
    );

    finalPlan.push(
      rangeCandidates['humidity-to-location'].find(
        (c) => c.seedRangeStart === min
      )?.seedRangeStart ?? 42
    );
  }
});
console.log('final values : ', finalPlan);
const min = Math.min(...finalPlan);
console.log('answer p1 => ', min);
