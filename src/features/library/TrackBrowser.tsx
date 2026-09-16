import { useRef, useEffect } from "react";
import { useVirtualizer } from "@tanstack/react-virtual";
import { useLibraryStore } from "./libraryStore";

export function TrackBrowser() {
  const { tracks, selectedTrackIds, selectTrack, loadTracks, isLoading, sortColumn, sortDirection, setSort } = useLibraryStore();
  const parentRef = useRef<HTMLDivElement>(null);

  // Load initial tracks
  useEffect(() => {
    loadTracks();
  }, [loadTracks]);

  const rowVirtualizer = useVirtualizer({
    count: tracks.length,
    getScrollElement: () => parentRef.current,
    estimateSize: () => 36, // 36px height per row
    overscan: 10,
  });

  const formatDuration = (ms: number): string => {
    const totalSeconds = Math.floor(ms / 1000);
    const minutes = Math.floor(totalSeconds / 60);
    const seconds = totalSeconds % 60;
    return `${minutes}:${seconds.toString().padStart(2, "0")}`;
  };

  const renderSortIcon = (column: string) => {
    if (sortColumn !== column) return null;
    return <span style={{ fontSize: 10, marginLeft: 4 }}>{sortDirection === 'asc' ? '▲' : '▼'}</span>;
  };

  if (isLoading && tracks.length === 0) {
    return (
      <div className="browser__empty">
        <div className="browser__empty-icon">⏳</div>
        <div className="browser__empty-title">Loading library...</div>
      </div>
    );
  }

  return (
    <div className="track-table-container">
      {/* Table Header */}
      <div className="track-table-header">
        <div className="th" style={{ width: 40 }}>#</div>
        <div className="th" style={{ flex: 2, cursor: 'pointer' }} onClick={() => setSort('title')}>
          Title {renderSortIcon('title')}
        </div>
        <div className="th" style={{ flex: 1.5, cursor: 'pointer' }} onClick={() => setSort('artist')}>
          Artist {renderSortIcon('artist')}
        </div>
        <div className="th" style={{ flex: 1.5, cursor: 'pointer' }} onClick={() => setSort('album')}>
          Album {renderSortIcon('album')}
        </div>
        <div className="th" style={{ width: 80, cursor: 'pointer' }} onClick={() => setSort('bpm')}>
          BPM {renderSortIcon('bpm')}
        </div>
        <div className="th" style={{ width: 60, cursor: 'pointer' }} onClick={() => setSort('musical_key')}>
          Key {renderSortIcon('musical_key')}
        </div>
        <div className="th" style={{ width: 80, cursor: 'pointer' }} onClick={() => setSort('duration_ms')}>
          Time {renderSortIcon('duration_ms')}
        </div>
      </div>

      {/* Virtualized Body */}
      <div
        ref={parentRef}
        className="track-table-body"
        style={{
          height: "100%",
          overflow: "auto",
        }}
      >
        <div
          style={{
            height: `${rowVirtualizer.getTotalSize()}px`,
            width: "100%",
            position: "relative",
          }}
        >
          {rowVirtualizer.getVirtualItems().map((virtualRow) => {
            const track = tracks[virtualRow.index];
            const isSelected = selectedTrackIds.has(track.id);

            return (
              <div
                key={virtualRow.index}
                className={`track-row ${isSelected ? "track-row--selected" : ""}`}
                style={{
                  position: "absolute",
                  top: 0,
                  left: 0,
                  width: "100%",
                  height: `${virtualRow.size}px`,
                  transform: `translateY(${virtualRow.start}px)`,
                }}
                onClick={(e) => selectTrack(track.id, e.ctrlKey || e.metaKey)}
              >
                <div className="td" style={{ width: 40, color: "var(--color-text-tertiary)" }}>
                  {virtualRow.index + 1}
                </div>
                <div className="td" style={{ flex: 2, fontWeight: 500, color: "var(--color-text-primary)" }}>
                  {track.metadata.title || track.filename}
                </div>
                <div className="td" style={{ flex: 1.5 }}>{track.metadata.artist || ""}</div>
                <div className="td" style={{ flex: 1.5 }}>{track.metadata.album || ""}</div>
                <div className="td" style={{ width: 80, fontFamily: "var(--font-mono)" }}>
                  {track.bpm ? track.bpm.toFixed(1) : "-"}
                </div>
                <div className="td" style={{ width: 60, fontFamily: "var(--font-mono)" }}>
                  {track.musical_key || "-"}
                </div>
                <div className="td" style={{ width: 80, fontFamily: "var(--font-mono)" }}>
                  {formatDuration(track.metadata.duration_ms)}
                </div>
              </div>
            );
          })}
        </div>
      </div>
    </div>
  );
}
