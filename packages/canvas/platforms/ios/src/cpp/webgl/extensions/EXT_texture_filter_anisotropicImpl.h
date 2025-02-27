//
// Created by Osei Fortune on 29/04/2022.
//

#pragma once

#include "gl.h"
#include <vector>
#include "Helpers.h"
#include "Common.h"
#include "Caches.h"
#include "ObjectWrapperImpl.h"
class EXT_texture_filter_anisotropicImpl: ObjectWrapperImpl {
public:
    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate) {
        auto cache = Caches::Get(isolate);
        auto ctor = cache->EXT_texture_filter_anisotropicTmpl.get();
        if (ctor != nullptr) {
            return ctor->Get(isolate);
        }

        v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
        ctorTmpl->InstanceTemplate()->SetInternalFieldCount(2);
        ctorTmpl->SetClassName(ConvertToV8String(isolate, "EXT_texture_filter_anisotropic"));

        auto tmpl = ctorTmpl->InstanceTemplate();
        tmpl->SetInternalFieldCount(2);
        tmpl->Set(ConvertToV8String(isolate, "MAX_TEXTURE_MAX_ANISOTROPY_EXT"),
                  v8::Integer::NewFromUnsigned(isolate, GL_MAX_TEXTURE_MAX_ANISOTROPY_EXT));
        tmpl->Set(ConvertToV8String(isolate, "TEXTURE_MAX_ANISOTROPY_EXT"),
                  v8::Integer::NewFromUnsigned(isolate, GL_TEXTURE_MAX_ANISOTROPY_EXT));
        tmpl->Set(ConvertToV8String(isolate, "ext_name"),
                  ConvertToV8String(isolate, "EXT_texture_filter_anisotropic"));

        cache->EXT_texture_filter_anisotropicTmpl =
                std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
        return ctorTmpl;
    }

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, EXT_texture_filter_anisotropicImpl *filterAnisotropic) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = EXT_texture_filter_anisotropicImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType( object, NativeType::EXT_texture_filter_anisotropic);
        object->SetAlignedPointerInInternalField(0, filterAnisotropic);
        filterAnisotropic->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static EXT_texture_filter_anisotropicImpl *GetPointer(const v8::Local<v8::Object> &object) {
        auto ptr = object->GetAlignedPointerFromInternalField(0);
        if (ptr == nullptr) {
            return nullptr;
        }
        return static_cast<EXT_texture_filter_anisotropicImpl *>(ptr);
    }
};
