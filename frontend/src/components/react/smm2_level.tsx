import { formatClearcheckMs, formatHHMMTime, formatISODate, formatSmm2LevelId, formatTagName } from "src/helpers";
import ThumbnailLoader from "./thumbnail_loader";

interface Smm2LevelProps {
  level: any; // [ToDO] I really should define this, but... lazy.
}

export default function Smm2Level({ level }: Smm2LevelProps) {
  let date = new Date(level.uploaded_at);
  return (
    <section className="box level-box">
      <h2 className="level-text">{level.title}</h2>
      <div className="thumbnail-and-metadata">
        <div className="thumbnail-container">
          <div className="thumbnail-container-inner">
            <ThumbnailLoader alt="Level Thumbnail" levelId={level.id} />
            <p>
              <span>{formatISODate(date)}</span>
              <span>{formatHHMMTime(date)}</span>
            </p>
          </div>
        </div>
        <div className="metadata-container">
          <ul className="counters">
            <li>
              <i className="fa-solid fa-heart" title="Likes"></i> {level.likes}
            </li>
            <li>
              <i className="fa-solid fa-heart-crack" title="Boos"></i> {level.boos}
            </li>
            <li>
              <i className="fa-solid fa-shoe-prints fa-rotate-270" title="Footprints"></i> {level.footprints}
            </li>
            <li>
              <i className="fa-solid fa-comments" title="Comments"></i> {level.comments}
            </li>
          </ul>
          <p>
            <i className="fa-solid fa-gamepad" title="Style"></i> {level.style}
          </p>
          <p>
            <i className="fa-solid fa-stopwatch" title="Clear check time"></i> {formatClearcheckMs(level.clearcheck_ms)}
          </p>
          {level.tags.length > 0 && (
            <p>
              <i className="fa-solid fa-tag" title="Tags"></i> {level.tags.map(formatTagName).join(", ")}
            </p>
          )}
        </div>
      </div>
      <div className="level-info">
        <div className="text-box level-text">
          <p>{level.description != null ? level.description : <>(no description)</>}</p>
        </div>
        <div className="two-col">
          <div className="text-box">
            <p className="header">Attempts</p>
            <p className="content-large">{level.attempts}</p>
          </div>
          <div className="text-box">
            <p className="header">Course ID</p>
            <p className="content-large">{formatSmm2LevelId(level.id)}</p>
          </div>
        </div>
      </div>
      <div className="level-actions">
        <a className="button" href={`https://smm2.wizul.us/smm2/level/${formatSmm2LevelId(level.id)}`} target="_blank">
          <i className="fa-solid fa-eye"></i> Open in Viewer
        </a>
        {/*
         * [ToDo]: This is currently commented out as there is no backend support.
         *   <button className="button">
         *     <i className="fa-solid fa-hand"></i> I'm working on it!
         *   </button>
         *   <button className="button">
         *     <i className="fa-solid fa-flag-pennant"></i> It's cleared!
         *   </button>
         */}
      </div>
    </section>
  );
}
