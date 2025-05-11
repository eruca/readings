// src/pages/literatureList/index.jsx
import { useState, useEffect, useContext } from 'react';
import Taro from '@tarojs/taro';
import { View, ScrollView, Text } from '@tarojs/components';
import { Sticky, Button, Loading, Empty, CellGroup, Cell } from '@antmjs/vantui'

import { LiteratureContext } from "../../context/LiteratureContext";
import { fetchLiteratureListApi, uploadPdfApi } from '../../utils/api';
import { FixedTabbar } from '../../Components/Tabbar';

export default function LiteratureList() {
    const [literature, setLiterature] = useState([]);
    const [isLoading, setIsLoading] = useState(false);
    const { setSelectedPdf } = useContext(LiteratureContext);

    const loadLiterature = async () => {
        setIsLoading(true);
        try {
            // **实际API调用**: const res = await Taro.request({ url: 'YOUR_BACKEND_LIST_URL' });
            const res = await fetchLiteratureListApi();
            if (res.success) {
                setLiterature(res.data);
            } else {
                Taro.showToast({ title: res.message || '加载失败', icon: 'none' });
            }
        } catch (error) {
            Taro.showToast({ title: '网络错误', icon: 'none' });
        } finally {
            setIsLoading(false);
        }
    };

    useEffect(() => {
        loadLiterature();
    }, []);

    const handleSelectPdf = (pdf) => {
        setSelectedPdf(pdf);
        Taro.navigateTo({ url: `/pages/literatureDetail/index?pdfId=${pdf.id}&pdfName=${encodeURIComponent(pdf.name)}` });
    };

    const handleUploadPdf = async () => {
        try {
            const chooseRes = await Taro.chooseMessageFile({
                count: 1,
                type: 'file',
                extension: ['pdf'],
            });

            if (chooseRes.tempFiles && chooseRes.tempFiles.length > 0) {
                const tempFilePath = chooseRes.tempFiles[0].path;
                const tempFileName = chooseRes.tempFiles[0].name;

                Taro.showLoading({ title: '上传中...' });
                // **实际API调用**:
                // const uploadRes = await Taro.uploadFile({
                //   url: 'YOUR_BACKEND_UPLOAD_URL', // 替换为你的Rust/Axum上传接口
                //   filePath: tempFilePath,
                //   name: 'pdfFile', // 与后端接收字段名一致
                // });
                // const backendResponse = JSON.parse(uploadRes.data);
                // if (uploadRes.statusCode === 200 && backendResponse.fileId) {
                //   Taro.showToast({ title: '上传成功!', icon: 'success' });
                //   loadLiterature(); // 重新加载列表
                // } else {
                //   Taro.showToast({ title: backendResponse.error || '上传失败', icon: 'none' });
                // }
                const mockUploadRes = await uploadPdfApi(tempFilePath, tempFileName); // 模拟API
                Taro.hideLoading();
                if (mockUploadRes.success) {
                    Taro.showToast({ title: '上传成功!', icon: 'success' });
                    // 可以在这里直接将 mockUploadRes.data 添加到列表，或重新调用 loadLiterature
                    setLiterature(prev => [mockUploadRes.data, ...prev.filter(item => item.id !== mockUploadRes.data.id)]);
                } else {
                    Taro.showToast({ title: mockUploadRes.message || '上传失败', icon: 'none' });
                }
            }
        } catch (error) {
            Taro.hideLoading();
            console.error('Upload error:', error);
            Taro.showToast({ title: '上传操作失败', icon: 'none' });
        }
    };

    return (
        <View className='flex flex-col'>
            <Sticky className='mx-2 mt-2'>
                <Button type='primary' block icon='plus' onClick={handleUploadPdf}>
                    上传新文献 (PDF)
                </Button>
            </Sticky>

            {isLoading && <Loading custom-class="loading-center" type="spinner" color="#1989fa" />}

            {!isLoading && literature.length === 0 && (
                <Empty description="暂无文献，快去上传吧" />
            )}

            <View className="flex-grow mt-2">
                <ScrollView scrollY >
                    <CellGroup inset>
                        {literature.map((pdf) => (
                            <Cell
                                key={pdf.id}
                                title={pdf.name}
                                value={pdf.qnaGenerated ? 'Q&A已生成' : 'Q&A未生成'}
                                label={`上传日期: ${pdf.uploadDate}`}
                                isLink
                                onClick={() => handleSelectPdf(pdf)}
                            />
                        ))}
                    </CellGroup>
                </ScrollView>
            </View>
            <FixedTabbar active={1} />
        </View>
    );
}