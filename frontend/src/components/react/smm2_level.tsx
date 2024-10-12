import {
  formatClearcheckMs,
  formatClearCondition,
  formatHHMMTime,
  formatISODate,
  formatSmm2LevelId,
  formatTagName,
} from "src/helpers";
import ThumbnailLoader from "./thumbnail_loader";
import ClickableCourseId from "./clickable_course_id";
import { useMutation } from "@tanstack/react-query";

interface Smm2LevelProps {
  level: any; // [ToDO] I really should define this, but... lazy.
}

function getMarkClearButtonLabel(status: string): string {
  switch (status) {
    case "pending":
      return "Working...";
    case "success":
      return "Done! :)";
    case "error":
      return "Failed! :(";
    default:
      return "Mark as Cleared";
  }
}

export default function Smm2Level({ level }: Smm2LevelProps) {
  const markClearMutation = useMutation({
    mutationKey: ["/smm2/mark_cleared", level.id],
    mutationFn: async () => {
      const headers = new Headers();
      headers.append("content-type", "application/json");

      const res = await fetch(`${import.meta.env.PUBLIC_SMM_ZEROP_API_ROOT}/smm2/mark_cleared`, {
        method: "POST",
        headers,
        body: JSON.stringify({
          level_id: level.id,
        }),
      });

      if (!res.ok) {
        const data = {
          status: "err",
          responseCode: res.status,
          data: await res.text(),
        };
        console.error("markClearMutation failed", data);
        return Promise.reject(data);
      }
    },
  });

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
          <ul className="inline">
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
          <ul className="inline">
            <li>
              <i className="fa-solid fa-gamepad" title="Game style"></i> {level.style}
            </li>
            <li>
              <i className="fa-solid fa-palette" title="Level Theme"></i> {formatTagName(level.theme)}
            </li>
          </ul>
          <p>
            <i className="fa-solid fa-stopwatch" title="Clear check time"></i> {formatClearcheckMs(level.clearcheck_ms)}
          </p>
          {level.tags.length > 0 && (
            <p>
              <i className="fa-solid fa-tag" title="Tags"></i> {level.tags.map(formatTagName).join(", ")}
            </p>
          )}
          {level.clear_condition && (
            <p>
              <i className="fa-solid fa-flag-checkered" title="Clear Condition"></i>{" "}
              {formatClearCondition(level.clear_condition, level.clear_condition_magnitude)}
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
          <ClickableCourseId levelId={formatSmm2LevelId(level.id)} />
        </div>
      </div>
      <div className="level-actions">
        <a className="button" href={`https://smm2.wizul.us/smm2/level/${formatSmm2LevelId(level.id)}`} target="_blank">
          <i className="fa-solid fa-eye"></i> Open in Viewer
        </a>
        <button
          className="button"
          disabled={["loading", "success"].includes(markClearMutation.status)}
          onClick={() => {
            if (["loading", "success"].includes(markClearMutation.status)) {
              return;
            }

            markClearMutation.mutate();
          }}
        >
          <i className="fa-solid fa-flag-pennant"></i> {getMarkClearButtonLabel(markClearMutation.status)}
        </button>
        {/*
         * [ToDo]: This is currently commented out as there is no backend support.
         *   <button className="button">
         *     <i className="fa-solid fa-hand"></i> I'm working on it!
         *   </button>
         */}
      </div>
    </section>
  );
}
