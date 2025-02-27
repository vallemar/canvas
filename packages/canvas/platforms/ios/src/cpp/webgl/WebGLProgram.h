//
// Created by Osei Fortune on 27/04/2022.
//

#pragma

#include "Common.h"
#include "ObjectWrapperImpl.h"

class WebGLProgram:ObjectWrapperImpl  {
public:
    WebGLProgram(uint32_t program) : program_(program) {}

    uint32_t GetProgram() {
        return this->program_;
    }

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate) {
        auto cache = Caches::Get(isolate);
        auto ctor = cache->WebGLProgramTmpl.get();
        if (ctor != nullptr) {
            return ctor->Get(isolate);
        }

        v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
        ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
        ctorTmpl->SetClassName(ConvertToV8String(isolate, "WebGLProgram"));

        auto tmpl = ctorTmpl->InstanceTemplate();
        tmpl->SetInternalFieldCount(2);

        cache->WebGLProgramTmpl =
                std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
        return ctorTmpl;
    }

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, WebGLProgram *program) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = WebGLProgram::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType( object, NativeType::WebGLProgram);
        object->SetAlignedPointerInInternalField(0, program);
        program->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static WebGLProgram *GetPointer(const v8::Local<v8::Object> &object) {
        auto ptr = object->GetAlignedPointerFromInternalField(0);
        if (ptr == nullptr) {
            return nullptr;
        }
        return static_cast<WebGLProgram *>(ptr);
    }

private:
    uint32_t program_;
};

