export class TrickFilmPlayer {
  constructor(trickFilm, trickFilmRegistion) {
    this.paused = false;
    this.speed = 1.0;
    this.trickFilm = trickFilm;
    this.trickFilmRegistion = trickFilmRegistion;
    this.server = simpleWarfareCli.trickFilmPlayerServer;
    this.registions = new Object();

    for (const registion of trickFilmRegistion) {
      this.registions[registion] = registion;
    }
  }

  start() {}

  play() {}
}
