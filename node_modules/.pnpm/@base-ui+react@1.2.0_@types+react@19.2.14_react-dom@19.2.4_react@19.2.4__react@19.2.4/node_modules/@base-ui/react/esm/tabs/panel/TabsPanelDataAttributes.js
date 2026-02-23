export let TabsPanelDataAttributes = /*#__PURE__*/function (TabsPanelDataAttributes) {
  /**
   * Indicates the index of the tab panel.
   */
  TabsPanelDataAttributes["index"] = "data-index";
  /**
   * Indicates the direction of the activation (based on the previous active tab).
   * @type {'left' | 'right' | 'up' | 'down' | 'none'}
   */
  TabsPanelDataAttributes["activationDirection"] = "data-activation-direction";
  /**
   * Indicates the orientation of the tabs.
   * @type {'horizontal' | 'vertical'}
   */
  TabsPanelDataAttributes["orientation"] = "data-orientation";
  /**
   * Present when the panel is hidden.
   */
  TabsPanelDataAttributes["hidden"] = "data-hidden";
  return TabsPanelDataAttributes;
}({});