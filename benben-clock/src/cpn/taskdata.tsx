export const columns = [
  {
    title: "任务名称",
    dataIndex: "name",
    align: "center",
    key: "name",
    width: 70,
    fixed: "left",
    customHeaderCell: () => {
      return {
        style: {
          padding: "5px",
        },
      };
    },
    customCell: () => {
      return {
        style: {
          padding: "5px",
        },
      };
    },
  },
  {
    title: "开始时间",
    dataIndex: "start_time",
    key: "start_time",
    width: 50,
    align: "center",
    customHeaderCell: () => {
      return {
        style: {
          padding: "5px",
        },
      };
    },
    customCell: () => {
      return { style: { padding: "5px" } };
    },
  },
  {
    title: "难度",
    key: "hard",
    align: "center",
    ellipsis: true,
    dataIndex: "hard",
    width: 28,
    slots: { customRender: "hards" },

    customHeaderCell: () => {
      return {
        style: {
          padding: "5px",
        },
      };
    },
    customCell: () => {
      return { style: { padding: "5px" } };
    },
  },
  {
    title: "优先级",
    key: "level",
    align: "center",

    customHeaderCell: () => {
      return {
        style: {
          padding: "5px",
        },
      };
    },
    customCell: () => {
      return { style: { padding: "5px" } };
    },
    dataIndex: "level",
    sorter: (
      a: { level: number; status: string },
      b: { level: number; status: string }
    ) =>
      (a.status === "0" || a.status === "1" || a.status === "2") &&
      a.level > b.level,
    width: 40,
    slots: { customRender: "levels" },
  },
  {
    title: "操作",
    ellipsis: true,
    dataIndex: "",
    key: "x",
    slots: { customRender: "action" },
    width: 60,
    align: "center",
    fixed: "right",
    customHeaderCell: () => {
      return {
        style: {
          padding: "5px",
        },
      };
    },
    customCell: () => {
      return { style: { padding: "8px" } };
    },
  },
];
