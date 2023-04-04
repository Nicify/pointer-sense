import { Input } from '@mantine/core'
import { exit } from '@tauri-apps/api/process'
import { useAtom } from 'jotai/react'

import { Button } from '@/components/Button'
import { Divider } from '@/components/Divider'
import { Header } from '@/components/Header'
import { useTranslation } from '@/i18n'
import { getWebviewWindow } from '@/lib/tauri'
import { accEnabledAtom, resetState, senAtom } from '@/store'

import * as SC from './styles'

const marks = [
    { value: 0, label: '0' },
    { value: 25, label: '25' },
    { value: 50, label: '50' },
    { value: 75, label: '75' },
    { value: 100, label: '100' }
]

export const handleOpenPreferences = async () => {
    const window = await getWebviewWindow('preferences')

    if (window.isSome()) {
        window.get().show()
        return
    }
    // eslint-disable-next-line no-console
    console.error('Failed to get preferences window')
}

export default function Main() {
    const [sen, setSen] = useAtom(senAtom)
    const [accEnabled, setAccEnabled] = useAtom(accEnabledAtom)

    const T = useTranslation()

    return (
        <SC.Container direction="column" justify="space-between">
            <Header>SensiMouse (beta)</Header>
            <SC.Content>
                <Input.Wrapper label={T.PREFERENCES()}>
                    <SC.xSlider size="lg" marks={marks} min={0} max={100} value={sen} onChange={setSen} />
                </Input.Wrapper>
                <Input.Wrapper label={T.ACCELERATION()}>
                    <SC.xSwitch
                        size="md"
                        onLabel="ON"
                        offLabel="OFF"
                        checked={accEnabled}
                        onChange={event => {
                            setAccEnabled(event.target.checked)
                        }}
                    />
                </Input.Wrapper>
            </SC.Content>
            <Divider />
            <SC.Footer gap={8} justify="flex-end" align="center">
                <Button onClick={handleOpenPreferences}>{T.PREFERENCES()}</Button>
                <Button onClick={() => resetState()}>{T.RESET()}</Button>
                <Button onClick={() => exit(0)}>{T.QUIT()}</Button>
            </SC.Footer>
        </SC.Container>
    )
}
