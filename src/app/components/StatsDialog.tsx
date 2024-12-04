import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogTrigger } from "@/components/ui/dialog"
import { Button } from "@/components/ui/button"
import { BarChart3 } from "lucide-react"
import { invoke } from '@tauri-apps/api/core'
import { useEffect, useState } from "react"

interface BoomStats {
  total_booms: number;
  rare_booms: number;
  daily_booms: Record<string, number>;
  average_interval: number;
  last_boom: number | null;
}

export function StatsDialog() {
  const [stats, setStats] = useState<BoomStats | null>(null);

  const fetchStats = async () => {
    const stats = await invoke<BoomStats>('get_boom_stats');
    setStats(stats);
  };

  useEffect(() => {
    fetchStats();
    const interval = setInterval(fetchStats, 5000);
    return () => clearInterval(interval);
  }, []);

  if (!stats) return null;

  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button variant="ghost" size="icon" className="text-zinc-400 hover:text-white">
          <BarChart3 className="h-5 w-5" />
        </Button>
      </DialogTrigger>
      <DialogContent className="bg-zinc-900 text-white border-zinc-800">
        <DialogHeader>
          <DialogTitle>Boom Statistics</DialogTitle>
        </DialogHeader>
        <div className="space-y-4">
          <div className="grid grid-cols-2 gap-4">
            <div>
              <h3 className="text-sm text-zinc-400">Total Booms</h3>
              <p className="text-2xl font-bold">{stats.total_booms}</p>
            </div>
            <div>
              <h3 className="text-sm text-zinc-400">Rare Booms</h3>
              <p className="text-2xl font-bold">{stats.rare_booms}</p>
            </div>
            <div>
              <h3 className="text-sm text-zinc-400">Average Interval</h3>
              <p className="text-2xl font-bold">
                {Math.round(stats.average_interval / 60)} min
              </p>
            </div>
          </div>
          
          <div>
            <h3 className="text-sm text-zinc-400 mb-2">Daily Booms</h3>
            <div className="space-y-2">
              {Object.entries(stats.daily_booms)
                .sort((a, b) => new Date(b[0]).getTime() - new Date(a[0]).getTime())
                .slice(0, 5)
                .map(([date, count]) => (
                  <div key={date} className="flex justify-between">
                    <span>{new Date(date).toLocaleDateString()}</span>
                    <span>{count}</span>
                  </div>
                ))}
            </div>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
} 