import { formatISODate, formatSmm1LevelId } from "src/helpers";
import ClickableCourseId from "./clickable_course_id";

interface Smm1LevelProps {
  level: any; // [ToDO] I really should define this, but... lazy.
}

export default function Smm1Level({ level }: Smm1LevelProps) {
  let date = new Date(level.uploaded_at);
  return (
    <section className="box level-box smm1">
      <h2 className="level-text">{level.title}</h2>
      <div className="thumbnail-and-metadata">
        <div className="metadata-container">
          <p>
            <i className="fa-solid fa-calendar-clock" title="Uploaded on"></i> {formatISODate(date)}
          </p>
          <p>
            <i className="fa-solid fa-heart" title="Likes"></i> {level.likes}
          </p>
          <p>
            <i className="fa-solid fa-shoe-prints fa-rotate-270" title="Footprints"></i> {level.footprints}
          </p>
        </div>
      </div>
      <div className="level-info">
        <div className="two-col">
          <div className="text-box">
            <p className="header">Attempts</p>
            <p className="content-large">{level.attempts}</p>
          </div>
          <ClickableCourseId levelId={formatSmm1LevelId(level.id)} />
        </div>
      </div>

      {/*
       * [ToDo]: This is currently commented out as there is no backend support.
       * <div className="level-actions">
       *   <button className="button">
       *     <i className="fa-solid fa-hand"></i> I'm working on it!
       *   </button>
       *   <button className="button">
       *     <i className="fa-solid fa-flag-pennant"></i> It's cleared!
       *   </button>
       * </div>
       */}
    </section>
  );
}
