import request from '../utils/request';

export namespace MaterialApi {
  export interface ListRequest {
    page: number;
    per_page: number;
  }

  export interface ListResponseItem {
      id: number;
      category_id: number;
      title: string;
      author: string;
      source: string;
      source_url: string;
      thumbnail: string;
      summary: string;
      created_at: string;
      updated_at: string;
  }

  export interface ListResponse {
    page: number;
    per_page: number;
    total_pages: number;
    data: ListResponseItem[];
  }

  export interface DetailResponse {
      id: number;
      category_id: number;
      title: string;
      author: string;
      source: string;
      source_url: string;
      thumbnail: string;
      summary: string;
      content: string;
      created_at: string;
      updated_at: string;
  }

  export interface CreateRequest {
      category_id: number;
      title: string;
      author: string;
      source: string;
      source_url: string;
      thumbnail: string;
      summary: string;
      content: string;
  }

  export interface CreateResponse {
    id: number;
  }

  export interface UpdateRequest {
      category_id: number;
      title: string;
      author: string;
      source: string;
      source_url: string;
      thumbnail: string;
      summary: string;
      content: string;
  }

  export interface UpdateResponse {
    id: number;
  }

  export interface RemoveRequest {
      id: number;
  }

  export interface RemoveResponse {
  }
}

const materialApi = {
  list: (data: MaterialApi.ListRequest) => {
    return request.get<MaterialApi.ListResponse>('/api/v1/materials', { params: data });
  },
  detail: (id: number) => {
    return request.get<MaterialApi.DetailResponse>(`/api/v1/materials/${id}`);
  },
  remove: (id: number) => {
    return request.delete<MaterialApi.RemoveResponse>(`/api/v1/materials/${id}`);
  },
  create: (data: MaterialApi.CreateRequest) => {
    return request.post<MaterialApi.CreateResponse>('/api/v1/materials', data);
  },
  update: (id: number, data: MaterialApi.UpdateRequest) => {
    return request.put<MaterialApi.UpdateResponse>(`/api/v1/materials/${id}`, data);
  },
};

export default materialApi;

