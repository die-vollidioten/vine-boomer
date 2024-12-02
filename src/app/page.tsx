"use client"

import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogTrigger } from "@/components/ui/dialog"
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"
import { HelpCircle, Loader2 } from "lucide-react"
import { invoke } from '@tauri-apps/api/core'
import { useState, useEffect } from 'react'
import toast, { Toaster } from 'react-hot-toast'

interface Status {
  enabled: boolean;
  min_time: number;
  max_time: number;
}

type TimeUnit = 'seconds' | 'minutes' | 'hours';

const TIME_MULTIPLIERS: Record<TimeUnit, number> = {
  seconds: 1,
  minutes: 60,
  hours: 3600,
};

export default function VineBoomer() {
  const [isEnabled, setIsEnabled] = useState(false)
  const [minTime, setMinTime] = useState(1)
  const [maxTime, setMaxTime] = useState(30)
  const [minTimeUnit, setMinTimeUnit] = useState<TimeUnit>('seconds')
  const [maxTimeUnit, setMaxTimeUnit] = useState<TimeUnit>('seconds')
  const [isUpdating, setIsUpdating] = useState(false)
  const [isAutostartEnabled, setIsAutostartEnabled] = useState(false)
  const [isLoading, setIsLoading] = useState(true)

  const updateStatus = async () => {
    try {
      const status = await invoke<Status>('get_status')
      setIsEnabled(status.enabled)
      setMinTime(Math.max(1, Math.round(status.min_time / TIME_MULTIPLIERS[minTimeUnit])))
      setMaxTime(Math.max(1, Math.round(status.max_time / TIME_MULTIPLIERS[maxTimeUnit])))
      
      const autostartEnabled = await invoke<boolean>('is_autostart_enabled')
      setIsAutostartEnabled(autostartEnabled)
    } catch (error) {
      console.error('Failed to get status:', error)
    } finally {
      setIsLoading(false)
    }
  }

  useEffect(() => {
    updateStatus()
  }, [])

  const handleToggleAutostart = async () => {
    try {
      if (isAutostartEnabled) {
        await invoke('disable_autostart')
        toast.success("Autostart disabled")
      } else {
        await invoke('enable_autostart')
        toast.success("Autostart enabled")
      }
      setIsAutostartEnabled(!isAutostartEnabled)
    } catch (error) {
      toast.error("Failed to toggle autostart")
      console.error('Failed to toggle autostart:', error)
    }
  }

  const handleToggle = async () => {
    try {
      setIsUpdating(true)
      const newStatus = await invoke<boolean>('toggle_status')
      setIsEnabled(newStatus)
    } catch (error) {
      console.error('Failed to toggle status:', error)
    } finally {
      setIsUpdating(false)
    }
  }

  const handleIntervalChange = async () => {
    const minInSeconds = minTime * TIME_MULTIPLIERS[minTimeUnit]
    const maxInSeconds = maxTime * TIME_MULTIPLIERS[maxTimeUnit]
    
    try {
      setIsUpdating(true)
      await invoke('set_interval', { 
        min: minInSeconds, 
        max: Math.max(minInSeconds, maxInSeconds)
      })
      toast.dismiss()
      toast.success("Interval updated successfully!")
    } catch (error) {
      toast.dismiss()
      toast.error("Failed to update interval")
      console.error('Failed to set interval:', error)
    } finally {
      setIsUpdating(false)
    }
  }

  const handleUnitChange = (value: TimeUnit, isMin: boolean) => {
    const oldUnit = isMin ? minTimeUnit : maxTimeUnit
    const oldValue = isMin ? minTime : maxTime
    const oldMultiplier = TIME_MULTIPLIERS[oldUnit]
    const newMultiplier = TIME_MULTIPLIERS[value]
    const conversionFactor = oldMultiplier / newMultiplier
    
    const newValue = Math.max(1, Math.round(oldValue * conversionFactor))
    
    if (isMin) {
      setMinTime(newValue)
      setMinTimeUnit(value)
    } else {
      setMaxTime(newValue)
      setMaxTimeUnit(value)
    }
  }

  if (isLoading) {
    return (
      <div className="min-h-screen bg-black flex items-center justify-center">
        <Loader2 className="h-8 w-8 text-purple-600 animate-spin" />
      </div>
    )
  }

  return (
    <div className="min-h-screen bg-black">
      <Toaster 
        position="bottom-center"
        toastOptions={{
          className: 'bg-zinc-800 text-white border border-zinc-700',
          duration: 2000,
          style: {
            background: '#27272a',
            color: '#ffffff',
            border: '1px solid #3f3f46',
          },
        }}
      />
      <Card className="h-screen bg-zinc-900 text-white border-none rounded-none">
        <CardHeader className="border-b border-zinc-800 flex flex-row items-center justify-between">
          <CardTitle className="text-center flex-grow relative">
            Vine Boomer
            {isUpdating && (
              <Loader2 className="h-4 w-4 animate-spin absolute -right-6 top-1/2 -translate-y-1/2" />
            )}
          </CardTitle>
          <Dialog>
            <DialogTrigger asChild>
              <Button variant="ghost" size="icon" className="text-zinc-400 hover:text-white">
                <HelpCircle className="h-5 w-5" />
              </Button>
            </DialogTrigger>
            <DialogContent className="bg-zinc-900 text-white border-zinc-800">
              <DialogHeader>
                <DialogTitle>How to use Vine Boomer</DialogTitle>
              </DialogHeader>
              <div className="space-y-4 text-sm text-zinc-400">
                <p>1. Set the minimum and maximum time intervals</p>
                <p>2. Choose your preferred time units for each interval</p>
                <p>3. Click Start to begin random vine boom sounds</p>
                <p>4. The sound will play randomly between your set intervals</p>
                <p>5. Click Stop to pause the sounds</p>
                <p className="text-yellow-500">Tip: Hover over the tray icon for a preview sound!</p>
              </div>
            </DialogContent>
          </Dialog>
        </CardHeader>
        <CardContent className="space-y-4 pt-6 max-w-md mx-auto">
          <div className="grid grid-cols-2 gap-4">
            <div className="space-y-2">
              <label className="text-sm text-zinc-400">Minimalzeit:</label>
              <div className="space-y-2">
                <Input
                  type="number"
                  min={1}
                  max={999}
                  value={minTime}
                  onChange={(e) => setMinTime(Math.max(1, Math.min(999, Number(e.target.value))))}
                  onBlur={handleIntervalChange}
                  className="bg-zinc-800 border-zinc-700"
                  disabled={isUpdating}
                />
                <Select 
                  value={minTimeUnit} 
                  onValueChange={(value: TimeUnit) => handleUnitChange(value, true)}
                  disabled={isUpdating}
                >
                  <SelectTrigger className="bg-zinc-800 border-zinc-700 text-white">
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent className="bg-zinc-800 border-zinc-700 text-white">
                    <SelectItem value="seconds">Seconds</SelectItem>
                    <SelectItem value="minutes">Minutes</SelectItem>
                    <SelectItem value="hours">Hours</SelectItem>
                  </SelectContent>
                </Select>
              </div>
            </div>
            <div className="space-y-2">
              <label className="text-sm text-zinc-400">Maximalzeit:</label>
              <div className="space-y-2">
                <Input
                  type="number"
                  min={1}
                  max={999}
                  value={maxTime}
                  onChange={(e) => setMaxTime(Math.max(1, Math.min(999, Number(e.target.value))))}
                  onBlur={handleIntervalChange}
                  className="bg-zinc-800 border-zinc-700"
                  disabled={isUpdating}
                />
                <Select 
                  value={maxTimeUnit} 
                  onValueChange={(value: TimeUnit) => handleUnitChange(value, false)}
                  disabled={isUpdating}
                >
                  <SelectTrigger className="bg-zinc-800 border-zinc-700 text-white">
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent className="bg-zinc-800 border-zinc-700 text-white">
                    <SelectItem value="seconds">Seconds</SelectItem>
                    <SelectItem value="minutes">Minutes</SelectItem>
                    <SelectItem value="hours">Hours</SelectItem>
                  </SelectContent>
                </Select>
              </div>
            </div>
          </div>
          <div className="text-sm text-zinc-400">
            Status: {isEnabled ? 'Aktiviert' : 'Deaktiviert'}
          </div>
          <Button
            className="w-full bg-purple-600 hover:bg-purple-700 disabled:opacity-50"
            onClick={handleToggle}
            disabled={isUpdating}
          >
            {isEnabled ? 'Stop' : 'Start'}
          </Button>
          <div className="flex items-center space-x-2 mt-4">
            <input
              type="checkbox"
              checked={isAutostartEnabled}
              onChange={handleToggleAutostart}
              className="form-checkbox h-5 w-5 text-purple-600"
            />
            <label className="text-sm text-zinc-400">Start on system startup</label>
          </div>
        </CardContent>
      </Card>
    </div>
  )
}
