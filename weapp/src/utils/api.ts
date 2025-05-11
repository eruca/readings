// src/utils/api.js

// 模拟文献列表
let mockLiteratureDB = [
  {
    id: "pdf1",
    name: "文献A：深度学习在医学影像中的应用.pdf",
    uploadDate: "2025-05-08",
    qnaGenerated: true,
  },
  {
    id: "pdf2",
    name: "文献B：COVID-19最新研究进展综述.pdf",
    uploadDate: "2025-05-09",
    qnaGenerated: false,
  },
];

// 模拟Q&A数据
const mockQnaDB = {
  pdf1: [
    {
      id: "q1",
      question: "深度学习在医学影像中主要有哪些应用方向？",
      answer: "主要包括图像分割、病灶检测、图像配准和辅助诊断等。",
    },
    {
      id: "q2",
      question: "什么是图像分割？",
      answer: "图像分割是指将医学影像中不同的组织或器官区域划分出来。",
    },
    // ... 8 more Q&A
  ],
  pdf2: [
    // Q&A for pdf2, or an empty array if not generated
  ],
};

export const fetchLiteratureListApi = () => {
  return new Promise((resolve) => {
    setTimeout(() => {
      resolve({ success: true, data: [...mockLiteratureDB] });
    }, 500);
  });
};

export const uploadPdfApi = (filePath, fileName) => {
  return new Promise((resolve) => {
    setTimeout(() => {
      const newPdf = {
        id: `pdf${mockLiteratureDB.length + 1}`,
        name: fileName,
        uploadDate: new Date().toISOString().split("T")[0],
        qnaGenerated: false, // 新上传的默认未生成Q&A
      };
      mockLiteratureDB.unshift(newPdf); //添加到列表顶部
      console.log(`Mock upload success for: ${fileName}, path: ${filePath}`);
      resolve({
        success: true,
        data: newPdf,
        message: "上传成功，正在处理...",
      });
    }, 1000);
  });
};

// 实际调用后端生成Q&A的接口
export const generateQnaApi = (pdfId) => {
  console.log(`Requesting Q&A generation for ${pdfId}`);
  return new Promise((resolve) => {
    setTimeout(() => {
      const pdf = mockLiteratureDB.find((p) => p.id === pdfId);
      if (pdf && pdf.id === "pdf2") {
        // 假设为pdf2生成Q&A
        mockQnaDB[pdfId] = [
          {
            id: "q2_1",
            question: "COVID-19的传播途径主要是什么？",
            answer: "主要通过呼吸道飞沫和密切接触传播。",
          },
          // ...其他问题
        ];
        pdf.qnaGenerated = true;
      }
      console.log(`Mock Q&A generation complete for ${pdfId}`);
      resolve({ success: true, message: `文献 ${pdfId} 的Q&A已生成完毕！` });
    }, 2000);
  });
};

export const fetchQnaListApi = (pdfId) => {
  return new Promise((resolve) => {
    setTimeout(() => {
      const qna = mockQnaDB[pdfId] || [];
      if (qna.length > 0) {
        resolve({ success: true, data: qna });
      } else {
        const pdf = mockLiteratureDB.find((p) => p.id === pdfId);
        if (pdf && !pdf.qnaGenerated) {
          resolve({
            success: false,
            data: [],
            message: "Q&A尚未生成，请先生成。",
            needsGeneration: true,
          });
        } else {
          resolve({ success: true, data: [], message: "暂无问答数据" });
        }
      }
    }, 500);
  });
};

export const sendMessageToLlmApi = (pdfId, message, history) => {
  console.log(
    `Sending to LLM for ${pdfId}: "${message}" with history:`,
    history
  );
  return new Promise((resolve) => {
    setTimeout(() => {
      let reply = "抱歉，我暂时无法回答这个问题。";
      if (message.includes("你好")) {
        reply = "你好！很高兴为您服务。";
      } else if (message.includes("总结")) {
        reply = `关于文献《${
          mockLiteratureDB.find((p) => p.id === pdfId)?.name
        }》的主要内容是...（此处为AI模拟总结）`;
      }
      resolve({ success: true, data: { answer: reply } });
    }, 800);
  });
};
