export function formatTimelineTick(ms: number): string {
    if (ms < 1000) return `${Math.round(ms)}ms`;
    const s = ms / 1000;
    if (s < 60) return `${s < 10 ? s.toFixed(1) : Math.round(s)}s`;
    const m = Math.floor(s / 60);
    const r = s - m * 60;
    return `${m}m ${Math.round(r)}s`;
}

export type TimelineTick = { pct: number; label: string; ms: number };

export function computeTimelineTicks(tMaxMs: number): TimelineTick[] {
    const max = Math.max(tMaxMs, 1);
    const target = 6;
    const step = Math.max(1, Math.ceil(max / target / 50) * 50);
    const out: TimelineTick[] = [];
    for (let t = 0; t <= max + 0.001; t += step) {
        out.push({
            ms: t,
            pct: Math.min(100, (t / max) * 100),
            label: formatTimelineTick(t),
        });
    }
    if (out.length === 0 || out[out.length - 1]!.pct < 99.5) {
        out.push({
            ms: max,
            pct: 100,
            label: formatTimelineTick(max),
        });
    }
    return out;
}
