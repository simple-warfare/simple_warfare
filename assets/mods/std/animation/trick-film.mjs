export class TrickFilmPlayer {
  constructor(graphicEntity, trickFilm, trickFilmRegistion) {
    this.entity = graphicEntity;
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

  start(registion) {
    this.server.start(this, registion);
  }

  play(registion) {
    this.server.play(this, registion);
  }
}
