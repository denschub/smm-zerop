import { useEffect, useRef, useState } from "react";

interface ClickableCourseIdProps {
  levelId: string;
}

export default function ClickableCourseId({ levelId }: ClickableCourseIdProps) {
  const payloadRef = useRef<HTMLParagraphElement>(null);
  const [popoverVisible, setPopoverVisible] = useState(false);
  const popoverTimerRef = useRef<number | undefined>(undefined);

  useEffect(() => {
    return () => {
      if (popoverTimerRef.current) {
        window.clearTimeout(popoverTimerRef.current);
      }
    };
  }, []);

  const boxClick: React.MouseEventHandler<HTMLDivElement> = () => {
    if (payloadRef.current) {
      navigator.clipboard.writeText(payloadRef.current.innerText.trim());

      setPopoverVisible(true);
      popoverTimerRef.current = window.setTimeout(() => {
        setPopoverVisible(false);
      }, 2000);
    }
  };

  return (
    <div className="text-box clickable-content" onClick={boxClick}>
      <div className="popover" style={popoverVisible ? undefined : { display: "none" }}>
        <p>Copied to clipboard!</p>
      </div>
      <p className="header">Course ID</p>
      <p className="content-large" ref={payloadRef}>
        {levelId}
      </p>
    </div>
  );
}
