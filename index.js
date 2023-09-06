import init, {add, div, goat_song} from './pkg/rust_wasm_101.js';

await init();

const addResult = add(999, 10);
console.log('What is addResult?', addResult);

const divideResult = div(999, 10);
console.log('What is divideResult?', divideResult);

const goatSongResult = goat_song(5);
console.log('What is goatSongResult?', goatSongResult);
