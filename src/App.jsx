import React, { useCallback, useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { Button, ButtonGroup, TextArea, HotkeysTarget2 as HotkeysTarget, Dialog, DialogBody, DialogFooter } from "@blueprintjs/core";
import { Column, Table2 as Table, Cell } from "@blueprintjs/table";
import '@blueprintjs/core/lib/css/blueprint.css';
import '@blueprintjs/icons/lib/css/blueprint-icons.css';
import "@blueprintjs/table/lib/css/table.css"
import "@blueprintjs/popover2/lib/css/blueprint-popover2.css";
// import { geocoder, carrier, timezones, parsePhoneNumberFromString } from '@devmehq/phone-number-validator-js'
import "./App.css";

const columnIndexToName = index => {
  switch (index) {
    case 0:
      return "digits";
    case 1:
      return "is_valid";
    case 2:
      return "carrier";
    case 3:
      return "email";
    case 4:
      return "international";
    case 5:
      return "national";
    case 6:
      return "e164";
    case 7:
      return "rfc3966";
    default:
      return null;
  }
}

const ControlPanel = () => (
  <ButtonGroup minimal>
    <Button icon="clipboard" />
    <Button icon="archive" />
  </ButtonGroup>
)

const Dump = ({ isOpen, onClose, text, setText }) => {


  const handleClick = useCallback(() => {
    invoke("parse", { text })
  }, [text])

  return (
    <Dialog isOpen={isOpen} onClose={onClose} canEscapeKeyClose={true} canOutsideClickClose={false}>
      <DialogBody>
        <TextArea onChange={e => setText(e.target.value)} value={text} style={{ minHeight: 300 }} fill={true} />
      </DialogBody>
      <DialogFooter actions={<Button disabled={text.trim().length === 0} intent="primary" text="Extract" onClick={handleClick} />} />
    </Dialog>
  )
}

function App() {
  const [numbers, setNumbers] = useState([]);
  const [newModalVisible, setNewModalVisible] = useState(false);
  const [text, setText] = useState("");

  const getCellData = (rowIndex, columnIndex) => {
    const number = numbers[rowIndex];
    if (!number) return null;
    const name = columnIndexToName(columnIndex);
    if (!name) return null;
    if (name === "is_valid") return number[name] ? "Yes" : "No";
    return number[name];
  };

  const cellRenderer = (rowIndex, columnIndex) => (
    <Cell>{getCellData(rowIndex, columnIndex)}</Cell>
  );

  useEffect(() => {
    listen('numbers::extracted', async ({payload: numbers}) => {
      setNumbers(numbers)
      setNewModalVisible(false)
      setText("")
    })
  }, [])

  const hotkeys = [
    {
      combo: "ctrl + n",
      global: true,
      label: "New data",
      onKeyDown: () => !newModalVisible && setNewModalVisible(true)
    },

    {
      combo: "ctrl + alt + shift + meta + w",
      global: true,
      label: "Wipe data",
      onKeyDown: () => {
        setNumbers([])
      }
    }
  ];

  return (
    <>
      <HotkeysTarget hotkeys={hotkeys}>
        {({ handleKeyDown, handleKeyUp }) => (
          <div className="container" onKeyDown={handleKeyDown} onKeyUp={handleKeyUp}>
            <Table numRows={numbers.length}>
              <Column
                cellRenderer={cellRenderer}
                key={"digits"}
                name={"Extracted"}
              />
              <Column
                cellRenderer={cellRenderer}
                key={"valid"}
                name={"Valid"}
              />
              <Column
                cellRenderer={cellRenderer}
                key={"Carrier"}
                name={"Carrier"}
              />
              <Column
                cellRenderer={cellRenderer}
                key={"Email"}
                name={"Email"}
              />
              <Column
                cellRenderer={cellRenderer}
                key={"International"}
                name={"International"}
              />
              <Column
                cellRenderer={cellRenderer}
                key={"National"}
                name={"National"}
              />
              <Column
                cellRenderer={cellRenderer}
                key={"E164"}
                name={"E164"}
              />
              <Column
                cellRenderer={cellRenderer}
                key={"Rfc3966"}
                name={"Rfc3966"}
              />
            </Table>

          </div>
        )}
      </HotkeysTarget>

      <Dump text={text} setText={setText} isOpen={newModalVisible} onClose={() => setNewModalVisible(false)} />
    </>
  );
}

export default App;
