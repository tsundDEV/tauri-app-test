import { listen } from '@tauri-apps/api/event'
import { useEffect, useState } from 'react'

export default function Dashboard() {
  const [eventDrop, setEventDrop] = useState<Object>({})
  useEffect(() => {
    listen('tauri://file-drop', setEventDrop)
  }, [setEventDrop])
  return (
    <div>
      Dashboard
      <p>
        {JSON.stringify(eventDrop)}
      </p>
    </div>
  )
}