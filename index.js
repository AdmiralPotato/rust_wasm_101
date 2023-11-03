import init, {
  add,
  div,
  goat_song,
  Goat,
  SuperGoat,
} from './pkg/rust_wasm_101.js';

await init();

const addResult = add(999, 10);
console.log('What is addResult?', addResult);

const divideResult = div(999, 10);
console.log('What is divideResult?', divideResult);

const goatSongResult = goat_song(5);
console.log('What is goatSongResult?', goatSongResult);

const goatLog = `
name: Gruff
power_level: 100
is_grumpy: true

name: Faun
power_level: 9001
is_grumpy: false

name: Billy
power_level: 100
is_grumpy: true
`;

const parsedGoats = JSON.parse(Goat.parse_log_as_json(goatLog));
console.log('What is parsedGoats?', parsedGoats);


const sillyGoatLog = `
power_level: 1001
is_grumpy: true
name: Backwards Goat

power_level: 900100
power_level: 10
is_grumpy: false
is_grumpy: true
is_grumpy: false
is_grumpy: true
name: Timmy the indecisive destroyer

power_level: 100
name: Flower
is_grumpy: false
`;

const parsedSillyGoats = JSON.parse(Goat.parse_log_as_json(sillyGoatLog));
console.log('What is parsedSillyGoats?', parsedSillyGoats);

const superGoat = SuperGoat.new();
console.log('What is superGoat?', superGoat.stringify());
