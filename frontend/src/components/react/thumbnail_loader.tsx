import { useState } from "react";

interface ThumbnailLoaderProps {
  alt: string;
  levelId: string;
}

export default function ThumbnailLoader({ alt, levelId }: ThumbnailLoaderProps) {
  const [loading, setLoading] = useState(true);
  const [errored, setErrored] = useState(false);

  return (
    <div className="thumbnail-loader">
      <div className="image-container">
        <img
          className={loading || errored ? "invisible" : ""}
          alt={alt}
          src={`https://tgrcode.com/mm2/level_thumbnail/${levelId}`}
          decoding="sync"
          onLoad={() => {
            setLoading(false);
          }}
          onError={() => {
            setLoading(false);
            setErrored(true);
          }}
        />
      </div>
      <div className="image-spinner-container">
        <div className="image-spinner">
          {loading && <i className="fa-duotone fa-fw fa-spinner-third fa-spin"></i>}
          {errored && <i className="fa-solid fa-fw fa-wifi-exclamation"></i>}
        </div>
      </div>
    </div>
  );
}
